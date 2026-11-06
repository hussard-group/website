import gsap from 'gsap';
import { ScrollToPlugin } from 'gsap/ScrollToPlugin';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger, ScrollToPlugin);

export { gsap, ScrollTrigger };

export const mm = gsap.matchMedia();

const REDUCED = '(prefers-reduced-motion: reduce)';

const cssTime = (name: string, fallback: number): number => {
    const raw = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
    const match = raw.match(/^([\d.]+)(m?s)$/);
    const value = Number.parseFloat(match?.[1] ?? '');
    return !match || Number.isNaN(value) ? fallback : match[2] === 'ms' ? value / 1000 : value;
};

export const scrollToY = (target: number): void => {
    if (window.matchMedia(REDUCED).matches) {
        window.scrollTo({ top: target, behavior: 'auto' });
        return;
    }
    const start = window.scrollY;
    const max = Math.max(0, document.documentElement.scrollHeight - window.innerHeight);
    const end = Math.min(Math.max(target, 0), max);
    gsap.to(window, {
        scrollTo: { y: end, autoKill: true },
        duration:
            cssTime('--transition-slow', 0.6) *
            Math.min(Math.max(Math.abs(end - start) / window.innerHeight, 0.5), 3),
        ease: 'power3.inOut',
        overwrite: 'auto',
    });
};

export const scrollToEl = (target: Element | string): void => {
    const el = typeof target === 'string' ? document.getElementById(target) : target;
    if (!el) return;
    if (window.matchMedia(REDUCED).matches) {
        el.scrollIntoView({ behavior: 'auto' });
        return;
    }
    const margin = Number.parseFloat(getComputedStyle(el).scrollMarginTop) || 0;
    scrollToY(el.getBoundingClientRect().top + window.scrollY - margin);
};

let configured = false;

export const initMotion = (): void => {
    if (typeof window === 'undefined' || configured) return;
    configured = true;
    ScrollTrigger.config({ ignoreMobileResize: true });
    document.fonts?.ready.then(() => ScrollTrigger.refresh());
};
