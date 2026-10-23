const THRESHOLD = 0.15;

export const reveal = (
    els: Iterable<Element>,
    win: Pick<Window, 'IntersectionObserver'> = window,
): IntersectionObserver | null => {
    const targets = [...els];
    if ('IntersectionObserver' in win) {
        const observer = new IntersectionObserver(
            (entries) => {
                for (const entry of entries) {
                    if (entry.isIntersecting) entry.target.classList.add('visible');
                }
            },
            { threshold: THRESHOLD },
        );
        for (const el of targets) observer.observe(el);
        return observer;
    }
    for (const el of targets) el.classList.add('visible');
    return null;
};
