import { gsap, initMotion } from './motion';

const MIN = 0.13;
const SPAN = 0.82;

const clamp01 = (v: number): number => Math.min(1, Math.max(0, v));

export const bindScrub = (
    els: Iterable<HTMLElement>,
    win: Pick<
        Window,
        'innerHeight' | 'addEventListener' | 'removeEventListener' | 'requestAnimationFrame'
    > = window,
): (() => void) => {
    const scenes = [...els].map((el) => ({
        el,
        words: [...el.querySelectorAll<HTMLElement>('.w')],
    }));

    let queued = false;

    const update = () => {
        queued = false;
        for (const { el, words } of scenes) {
            const rect = el.getBoundingClientRect();
            const run = rect.height - win.innerHeight;
            const p = run > 0 ? clamp01(-rect.top / run) : rect.top < win.innerHeight ? 1 : 0;
            const n = words.length;
            const lit = Math.min(1, p / SPAN) * n;
            for (let i = 0; i < n; i++) {
                words[i].style.opacity = String(MIN + (1 - MIN) * clamp01(lit - i));
            }
        }
    };

    const onScroll = () => {
        if (queued) return;
        queued = true;
        win.requestAnimationFrame(update);
    };

    win.addEventListener('scroll', onScroll, { passive: true });
    win.addEventListener('resize', onScroll, { passive: true });
    update();

    return () => {
        win.removeEventListener('scroll', onScroll);
        win.removeEventListener('resize', onScroll);
    };
};

const ZOOM_END = 32;
const FADE_IN = 0.55;
const FADE_SPAN = 0.37;
const WORD_W = 0.82;

export const bindZoom = (els: Iterable<HTMLElement>): (() => void) => {
    initMotion();
    const local = gsap.matchMedia();
    const unbinds: (() => void)[] = [];
    for (const el of els) {
        const pin = el.querySelector<HTMLElement>('.zoom-pin');
        const text = el.querySelector<SVGTextElement>('.zoom-word-text');
        const full = el.querySelector('.zoom-full');
        if (!pin || !text) continue;

        for (const v of el.querySelectorAll<HTMLVideoElement>('video')) v.play().catch(() => {});

        const ctx = document.createElement('canvas').getContext('2d');
        const center = { x: 0, y: 0 };
        const layout = () => {
            if (!ctx) return;
            const { width: w, height: h } = pin.getBoundingClientRect();
            const cs = getComputedStyle(text);
            ctx.font = `${cs.fontWeight} 100px ${cs.fontFamily}`;
            if ('letterSpacing' in ctx) ctx.letterSpacing = cs.letterSpacing;
            const wordW = ctx.measureText(text.textContent ?? '').width || 1;
            text.setAttribute('font-size', String(Math.min((WORD_W * w) / wordW, h / 72) * 100));
            center.x = w / 2;
            center.y = h / 2;
            text.setAttribute('x', String(center.x));
            text.setAttribute('y', String(center.y));
        };
        layout();
        window.addEventListener('resize', layout, { passive: true });
        unbinds.push(() => window.removeEventListener('resize', layout));

        local.add('(prefers-reduced-motion: no-preference)', () => {
            const tl = gsap.timeline({
                scrollTrigger: {
                    trigger: el,
                    start: 'top top',
                    end: 'bottom bottom',
                    scrub: 0.4,
                    invalidateOnRefresh: true,
                    onRefreshInit: layout,
                },
            });
            tl.fromTo(
                text,
                { scale: 1, svgOrigin: () => `${center.x} ${center.y}` },
                { scale: ZOOM_END, ease: 'power2.in', duration: 1 },
                0,
            );
            if (full)
                tl.fromTo(
                    full,
                    { opacity: 0 },
                    { opacity: 1, ease: 'none', duration: FADE_SPAN },
                    FADE_IN,
                );
            return () => tl.kill();
        });
    }
    return () => {
        for (const u of unbinds) u();
        local.revert();
    };
};
