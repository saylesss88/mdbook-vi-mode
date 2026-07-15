(function () {
  if (window.__viModeLoaded) return;   // injected on every page; guard once
  window.__viModeLoaded = true;

  const ZONE_KEY = 'vi-mode-zone';
  let zone = sessionStorage.getItem(ZONE_KEY) || 'content';
  const idx = { sidebar: 0, content: 0 };
  let cursorEl = null;
  let pendingG = false, gTimer = null;

  const CONTENT_SEL =
    '#mdbook-content main :is(h1,h2,h3,h4,h5,h6,p,li,pre,blockquote,table)';

  const isVisible = (el) => {
    const r = el.getBoundingClientRect();
    return r.width > 0 && r.height > 0;
  };

  // Queried live every time: the sidebar is populated async by toc.js.
  const targets = (z) =>
    z === 'sidebar'
      ? Array.from(document.querySelectorAll('#mdbook-sidebar a')).filter(isVisible)
      : Array.from(document.querySelectorAll(CONTENT_SEL))
          .filter((el) => isVisible(el) && el.textContent.trim().length);

  const clearCursor = () => {
    if (cursorEl) cursorEl.classList.remove('vi-cursor');
    cursorEl = null;
  };

  const paint = () => {
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
    idx[zone] = (idx[zone] + delta + n) % n;   // wrap around
    paint();
  };

  const currentChapterIndex = () => {
    const list = targets('sidebar');
    const path = location.pathname;
    const i = list.findIndex((a) => a.getAttribute('href') &&
      new URL(a.href).pathname === path);
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

  const onKey = (e) => {
    const t = e.target;
    if (t && t.matches('input, textarea, [contenteditable]')) return;
    if (e.ctrlKey || e.metaKey || e.altKey) return;

    switch (e.key) {
      case 'j': case 'ArrowDown': move(1); break;
      case 'k': case 'ArrowUp':   move(-1); break;
      case 'h': setZone('sidebar'); break;
      case 'l': setZone('content'); break;
      case 'G': idx[zone] = targets(zone).length - 1; paint(); break;
      case 'g':
        if (pendingG) { clearTimeout(gTimer); pendingG = false; idx[zone] = 0; paint(); }
        else { pendingG = true; gTimer = setTimeout(() => (pendingG = false), 500); }
        return;
      case 'Enter': case 'o': activate(); break;
      case 'Escape': clearCursor(); return;
      default: return;   // leave every other key alone
    }
    e.preventDefault();
  };

  const boot = () => {
    document.addEventListener('keydown', onKey, true);
    const sb = document.getElementById('mdbook-sidebar');
    if (sb) {
      // Rebuild once toc.js fills the scrollbox in.
      new MutationObserver(() => { if (zone === 'sidebar') paint(); })
        .observe(sb, { childList: true, subtree: true });
    }
    if (zone === 'sidebar') idx.sidebar = currentChapterIndex();
    paint();
  };

  if (document.readyState === 'loading')
    document.addEventListener('DOMContentLoaded', boot);
  else boot();
})();
