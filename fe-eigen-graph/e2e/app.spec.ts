import {test, expect} from '@playwright/test';

test('should allow token selection and update the UI', async ({page}) => {
    // 1. Navigate to the root page
    await page.goto('/');

    // 2. Assert that the page redirects to the default token '/eigen'
    await expect(page).toHaveURL(/.*\/eigen/);

    // 3. Assert that the "EigenLayer" card is highlighted
    const eigenCard = page.locator('button', {hasText: /EIGEN.*EigenLayer/});
    await expect(eigenCard).toHaveClass(/ring-2/);

    // 4. Click on a different token card (e.g., 'ALT')
    const altCard = page.locator('button', {hasText: /ALT.*AltLayer/});
    await altCard.click();

    // 5. Assert that the URL has updated to '/alt'
    await expect(page).toHaveURL(/.*\/alt/);

    // 6. Assert that the "AltLayer" card is now highlighted
    await expect(altCard).toHaveClass(/ring-2/);

    // 7. Assert that the content has updated using a more specific locator
    const operatorsTable = page.getByRole('heading', {name: 'Operators', exact: true});
    await expect(operatorsTable).toBeVisible();
});