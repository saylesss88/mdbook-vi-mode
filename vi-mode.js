// mdbook-vi-mode: client-side Vim-style navigation.
//
// Two states:
//   reading  - no cursor; every key passes through to the browser and mdBook.
//   nav      - a cursor is shown; j/k/h/l/gg/G/Enter are captured.
// The toggle key flips between them; Escape always returns to reading.
//
// This file is self-contained and carries its own defaults. The preprocessor
// may override them by defining `window.__viModeConfig` before this runs.
(function () {
  if (window.__viModeLoaded) return; // injected on every page; run once
  window.__viModeLoaded = true;

  const CFG = Object.assign(
    { toggleKey: '`', startActive: false, cursorColor: '#e46876' },
    window.__viModeConfig || {},
  );
  document.documentElement.style.setProperty('--vi-cursor', CFG.cursorColor);

  const ZONE_KEY = 'vi-mode-zone';
  const ACTIVE_KEY = 'vi-mode-active';

  const storedActive = sessionStorage.getItem(ACTIVE_KEY);
  let active = storedActive === null ? CFG.startActive : storedActive === 'true';
  let zone = sessionStorage.getItem(ZONE_KEY) || 'content';
  const idx = { sidebar: 0, content: 0 };
  let cursorEl = null;
  let pendingG = false;
  let gTimer = null;

  const CONTENT_SEL =
    '#mdbook-content main :is(h1,h2,h3,h4,h5,h6,p,li,pre,blockquote,table)';

  const isVisible = (el) => {
    const r = el.getBoundingClientRect();
    return r.width > 0 && r.height > 0;
  };

  // Queried live every time: the sidebar is populated asynchronously by toc.js.
  const targets = (z) =>
    z === 'sidebar'
      ? Array.from(document.querySelectorAll('#mdbook-sidebar a')).filter(isVisible)
      : Array.from(document.querySelectorAll(CONTENT_SEL)).filter(
          (el) => isVisible(el) && el.textContent.trim().length,
        );

  const clearCursor = () => {
    if (cursorEl) cursorEl.classList.remove('vi-cursor');
    cursorEl = null;
  };

  const SCROLL_STEP = 64; // px per j/k press in reading mode

  // mdBook may scroll the window or an inner #mdbook-content element depending
  // on viewport and theme; pick whichever is actually scrollable.
  const scroller = () => {
    const c = document.getElementById('mdbook-content');
    if (c && c.scrollHeight > c.clientHeight + 1) return c;
    return document.scrollingElement || document.documentElement;
  };

  const scrollByPx = (dy, smooth) =>
    scroller().scrollBy({ top: dy, behavior: smooth ? 'smooth' : 'auto' });

  const scrollToEdge = (bottom) => {
    const el = scroller();
    el.scrollTo({ top: bottom ? el.scrollHeight : 0, behavior: 'smooth' });
  };

  const paint = () => {
    if (!active) return;
    const list = targets(zone);
    if (!list.length) return;
    idx[zone] = Math.max(0, Math.min(idx[zone], list.length - 1));
    clearCursor();
    cursorEl = list[idx[zone]];
    cursorEl.classList.add('vi-cursor');
    cursorEl.scrollIntoView({ block: 'center', behavior: 'smooth' });
    sessionStorage.setItem(ZONE_KEY, zone);
  };

  const move = (delta) => {
    const n = targets(zone).length;
    if (!n) return;
    idx[zone] = (idx[zone] + delta + n) % n; // wrap around
    paint();
  };

  const currentChapterIndex = () => {
    const list = targets('sidebar');
    const path = location.pathname;
    const i = list.findIndex(
      (a) => a.getAttribute('href') && new URL(a.href).pathname === path,
    );
    return i >= 0 ? i : idx.sidebar;
  };

  const setZone = (z) => {
    if (zone === z) return;
    zone = z;
    if (zone === 'sidebar') idx.sidebar = currentChapterIndex();
    paint();
  };

  const activate = () => {
    const el = targets(zone)[idx[zone]];
    if (!el) return;
    if (zone === 'sidebar') return el.click();
    const link = el.tagName === 'A' ? el : el.querySelector('a');
    if (link) link.click();
  };

  const setActive = (on) => {
    active = on;
    sessionStorage.setItem(ACTIVE_KEY, String(on));
    document.body.classList.toggle('vi-mode-active', on);
    if (on) {
      if (zone === 'sidebar') idx.sidebar = currentChapterIndex();
      paint();
    } else {
      clearCursor();
    }
  };

  // Reading mode: Vim-style scrolling with no cursor. Keys the browser already
  // handles (arrows, space, PageUp/PageDown) are deliberately left untouched.
  const handleReadingKey = (e) => {
    switch (e.key) {
      case 'j':
        scrollByPx(SCROLL_STEP);
        break;
      case 'k':
        scrollByPx(-SCROLL_STEP);
        break;
      case 'd':
        scrollByPx(scroller().clientHeight / 2, true);
        break;
      case 'u':
        scrollByPx(-scroller().clientHeight / 2, true);
        break;
      case 'G':
        scrollToEdge(true);
        break;
      case 'g':
        if (pendingG) {
          clearTimeout(gTimer);
          pendingG = false;
          scrollToEdge(false);
        } else {
          pendingG = true;
          gTimer = setTimeout(() => (pendingG = false), 500);
        }
        return;
      default:
        return; // not ours; let it through
    }
    e.preventDefault();
  };

  const onKey = (e) => {
    const t = e.target;
    if (t && t.matches('input, textarea, [contenteditable]')) return;
    if (e.ctrlKey || e.metaKey || e.altKey) return;

    // The toggle key works in both states; Escape always returns to reading.
    if (e.key === CFG.toggleKey) {
      setActive(!active);
      e.preventDefault();
      return;
    }
    if (e.key === 'Escape') {
      if (active) {
        setActive(false);
        e.preventDefault();
      }
      return;
    }
    if (!active) {
      handleReadingKey(e);
      return;
    }

    switch (e.key) {
      case 'j':
      case 'ArrowDown':
        move(1);
        break;
      case 'k':
      case 'ArrowUp':
        move(-1);
        break;
      case 'h':
        setZone('sidebar');
        break;
      case 'l':
        setZone('content');
        break;
      case 'G':
        idx[zone] = targets(zone).length - 1;
        paint();
        break;
      case 'g':
        if (pendingG) {
          clearTimeout(gTimer);
          pendingG = false;
          idx[zone] = 0;
          paint();
        } else {
          pendingG = true;
          gTimer = setTimeout(() => (pendingG = false), 500);
        }
        return;
      case 'Enter':
      case 'o':
        activate();
        break;
      default:
        return; // leave every other key alone
    }
    e.preventDefault();
  };

  const boot = () => {
    const indicator = document.createElement('div');
    indicator.className = 'vi-mode-indicator';
    indicator.textContent = 'VI';
    document.body.appendChild(indicator);

    document.addEventListener('keydown', onKey, true);

    const sidebar = document.getElementById('mdbook-sidebar');
    if (sidebar) {
      // The sidebar is filled in asynchronously by toc.js; repaint when it lands.
      new MutationObserver(() => {
        if (active && zone === 'sidebar') paint();
      }).observe(sidebar, { childList: true, subtree: true });
    }

    document.body.classList.toggle('vi-mode-active', active);
    if (active) {
      if (zone === 'sidebar') idx.sidebar = currentChapterIndex();
      paint();
    }
  };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', boot);
  } else {
    boot();
  }
})();
