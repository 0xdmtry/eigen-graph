import React from 'react';
import {render, screen, fireEvent} from '@testing-library/react';
import TokenAutocomplete from '../TokenAutocomplete';
import {TokenProvider} from '@/context/TokenContext';

jest.mock('@/data/tokens', () => ({
    baseTokenCards: [
        {symbol: 'EIGEN', name: 'EigenLayer', icon: 'eigen.png', tvl: '', operators: 0},
        {symbol: 'ALT', name: 'AltLayer', icon: 'alt.png', tvl: '', operators: 0},
        {symbol: 'ETH', name: 'Ethereum', icon: 'eth.png', tvl: '', operators: 0},
    ],
}));

describe('TokenAutocomplete', () => {

    const renderComponent = () => {
        return render(
            <TokenProvider>
                <TokenAutocomplete/>
            </TokenProvider>
        );
    };

    it('renders the input with a placeholder', () => {
        renderComponent();
        const input = screen.getByPlaceholderText('Select Token...');
        expect(input).toBeInTheDocument();
    });

    it('shows all suggestions on focus', () => {
        renderComponent();
        const input = screen.getByPlaceholderText('Select Token...');
        fireEvent.focus(input);

        expect(screen.getByText('EigenLayer')).toBeInTheDocument();
        expect(screen.getByText('AltLayer')).toBeInTheDocument();
        expect(screen.getByText('Ethereum')).toBeInTheDocument();
    });

    it('filters suggestions when a user types', () => {
        renderComponent();
        const input = screen.getByPlaceholderText('Select Token...');
        fireEvent.change(input, {target: {value: 'alt'}});

        expect(screen.getByText('AltLayer')).toBeInTheDocument();
        expect(screen.queryByText('EigenLayer')).not.toBeInTheDocument();
        expect(screen.queryByText('Ethereum')).not.toBeInTheDocument();
    });

    it('selects a token and closes dropdown on suggestion click', async () => {
        renderComponent();
        const input = screen.getByPlaceholderText('Select Token...') as HTMLInputElement;

        fireEvent.focus(input);
        const altLayerOption = await screen.findByText('AltLayer');
        fireEvent.click(altLayerOption);

        expect(input.value).toBe('AltLayer');

        expect(screen.queryByText('EigenLayer')).not.toBeInTheDocument();
    });

    it('navigates and selects with keyboard', async () => {
        renderComponent();
        const input = screen.getByPlaceholderText('Select Token...') as HTMLInputElement;

        fireEvent.focus(input);
        expect(await screen.findByText('EigenLayer')).toBeInTheDocument();

        fireEvent.keyDown(input, {key: 'ArrowDown'});
        fireEvent.keyDown(input, {key: 'ArrowDown'});

        fireEvent.keyDown(input, {key: 'Enter'});

        expect(input.value).toBe('AltLayer');

        expect(screen.queryByText('EigenLayer')).not.toBeInTheDocument();
    });
});