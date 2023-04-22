pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut answer = 0;

    let mut minimum_price_before = prices[0];
    for selling_day_price in prices {
        let most_profit_here = selling_day_price - minimum_price_before;
        answer = std::cmp::max(answer, most_profit_here);
        minimum_price_before = std::cmp::min(minimum_price_before, selling_day_price);
    }

    answer
}
