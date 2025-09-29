export function formatScientific(value: string): string {
    try {
        const num = BigInt(value);
        if (num === BigInt(0)) {
            return '0.00e+0';
        }
        return Number(num).toExponential(2);
    } catch {
        return 'NaN';
    }
}

export function formatPowerOfTen(value: string): string {
    try {
        const num = BigInt(value);
        if (num === BigInt(0)) {
            return '0.00';
        }
        const exponential = Number(num).toExponential(2);
        const [mantissa, exponent] = exponential.split('e');

        const superscriptMap: Record<string, string> = {
            '0': '⁰', '1': '¹', '2': '²', '3': '³', '4': '⁴',
            '5': '⁵', '6': '⁶', '7': '⁷', '8': '⁸', '9': '⁹',
            '+': '', '-': '⁻'
        };

        const formattedExponent = exponent
            .split('')
            .map(char => superscriptMap[char])
            .join('');

        return `${mantissa} × 10${formattedExponent}`;
    } catch {
        return 'NaN';
    }
}

export function formatCompact(value: string): string {
    try {
        const num = BigInt(value);
        return new Intl.NumberFormat('en-US', {
            notation: 'compact',
            maximumFractionDigits: 2,
        }).format(num);
    } catch {
        return 'NaN';
    }
}