import '@testing-library/jest-dom';

jest.mock('next/navigation', () => ({
    useRouter: () => ({
        push: jest.fn(),
    }),
    useParams: () => ({
        tokenSymbol: 'EIGEN',
    }),
    usePathname: () => '/',
}));

jest.mock('next/image', () => ({
    __esModule: true,
    default: (props) => {
        return <img {...props} />;
    },
}));

window.HTMLElement.prototype.scrollIntoView = jest.fn();