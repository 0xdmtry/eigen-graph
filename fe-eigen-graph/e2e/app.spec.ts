import {test, expect} from "@playwright/test";

const apiPath = "**/v1/operators/aggregates";

const mockPayload = {
    meta: {source: "test", first: 0, skip: 0, count: 1},
    table: [],
    bar: [],
    donut: {},
    graph: [],
    outliers: {highConcentration: [], zeroShare: [], recentSlashes: []},
    byToken: {
        EIGEN: {
            meta: {source: "test", first: 0, skip: 0, count: 1},
            table: [
                {
                    operatorId: "0xabc",
                    avsCount: 2,
                    strategyCount: 3,
                    slashingCount: 0,
                    lastSlashAt: null,
                    lastUpdateBlockTs: 1700000000,
                    tvlTotalAtomic: "1000000000000000000",
                    hhiStrategy: 0,
                    nonzeroStrategyCount: 1
                }
            ],
            bar: [{operatorId: "0xabc", tvlTotalAtomic: "1000000000000000000"}],
            donut: {},
            graph: [{operatorId: "0xabc", strategyId: "0xdef", weightAtomic: "1000"}],
            outliers: {highConcentration: [], zeroShare: [], recentSlashes: []}
        }
    }
};

test.describe("token page", () => {
    test("valid token renders with server data", async ({page}) => {
        await page.route(apiPath, r => r.fulfill({status: 200, body: JSON.stringify(mockPayload)}));
        await page.goto("/eigen");

        const main = page.getByRole("main");
        await expect(main.getByRole("heading", {name: /^All Tokens/})).toBeVisible();
        await expect(main.getByRole("heading", {name: "Top Operators by TVL"})).toBeVisible();

        await expect(main.locator("table")).toBeVisible();
        await expect(main.locator("thead th").filter({hasText: /^Operator ID$/})).toHaveCount(1);
        await expect(main.getByPlaceholder("Search by Operator ID...")).toBeVisible();
    });

    test("invalid token 404", async ({page}) => {
        await page.goto("/invalidtoken", {waitUntil: "domcontentloaded"});
        await expect(page.getByRole("heading", {name: "ERROR"})).toBeVisible();
    });
});
