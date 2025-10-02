import nextJest from 'next/jest.js';

const createJestConfig = nextJest({dir: './'});

export default createJestConfig({
    testEnvironment: 'jest-environment-jsdom',
    setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
    moduleNameMapper: {'^@/(.*)$': '<rootDir>/src/$1'},
    testPathIgnorePatterns: [
        '/node_modules/',
        '/e2e/',
        '/temp/',
        '/playwright-report/',
        '/test-results/'
    ]
});
