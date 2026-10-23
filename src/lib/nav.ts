const THRESHOLD = 40;

export const isScrolled = (y: number): boolean => y > THRESHOLD;

export const bindNav = (
    el: HTMLElement | null,
    win: Pick<Window, 'scrollY' | 'addEventListener' | 'removeEventListener'> = window,
): (() => void) => {
    let scrolled = false;
    const onScroll = () => {
        if (!el) return;
        const next = isScrolled(win.scrollY);
        if (next === scrolled) return;
        scrolled = next;
        el.classList.toggle('scrolled', scrolled);
    };
    win.addEventListener('scroll', onScroll, { passive: true });
    return () => win.removeEventListener('scroll', onScroll);
};
