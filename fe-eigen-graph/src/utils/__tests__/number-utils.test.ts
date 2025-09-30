import {formatCompact, formatPowerOfTen, formatScientific} from '../number-utils';

const largeNumber = "123456789012345678901234567890123456789";
const zero = "0";
const invalidInput = "not a number";

describe('formatScientific', () => {
    it('formats a large number correctly', () => {
        expect(formatScientific(largeNumber)).toBe('1.23e+38');
    });

    it('handles zero correctly', () => {
        expect(formatScientific(zero)).toBe('0.00e+0');
    });

    it('handles invalid input', () => {
        expect(formatScientific(invalidInput)).toBe('NaN');
    });
});

describe('formatPowerOfTen', () => {
    it('formats a large number correctly', () => {
        expect(formatPowerOfTen(largeNumber)).toBe('1.23 × 10³⁸');
    });

    it('handles zero correctly', () => {
        expect(formatPowerOfTen(zero)).toBe('0.00');
    });

    it('handles invalid input', () => {
        expect(formatPowerOfTen(invalidInput)).toBe('NaN');
    });
});

describe('formatCompact', () => {
    const parseCompactNumber = (compactStr: string): number => {
        if (compactStr === 'NaN' || compactStr === '0') return Number(compactStr);
        const suffixMap: { [key: string]: number } = {
            K: 1e3, M: 1e6, B: 1e9, T: 1e12, Q: 1e15,
        };
        const numPart = parseFloat(compactStr);
        const suffix = compactStr.replace(/[\d.,\s-]/g, '');
        return suffix ? numPart * (suffixMap[suffix] || 1) : numPart;
    };

    it('formats a large number with a high degree of accuracy', () => {
        const input = "1234567890123456";
        const originalNumber = Number(input);
        const formatted = formatCompact(input);
        const parsedValue = parseCompactNumber(formatted);
        const relativeError = Math.abs(originalNumber - parsedValue) / originalNumber;
        expect(relativeError).toBeLessThan(0.001);
    });

    it('formats a smaller number correctly', () => {
        expect(formatCompact("54321")).toBe('54.32K');
    });

    it('handles zero correctly', () => {
        expect(formatCompact(zero)).toBe('0');
    });

    it('handles invalid input', () => {
        expect(formatCompact(invalidInput)).toBe('NaN');
    });
});