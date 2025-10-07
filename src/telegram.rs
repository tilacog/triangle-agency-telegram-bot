use teloxide::{
    RequestError,
    dispatching::UpdateHandler,
    prelude::*,
    types::{
        InlineQueryResult,
        InlineQueryResultArticle,
        InputMessageContent,
        InputMessageContentText,
    },
};

use crate::rng::create_rng;

async fn inline_query_handler(bot: Bot, q: InlineQuery) -> ResponseResult<()> {
    let mut rng = create_rng(q.id.to_string());
    let outcome = crate::dice::roll(&mut rng);

    let result = InlineQueryResultArticle::new(
        "roll_6d4",
        "🎲 Roll 6d4",
        InputMessageContent::Text(InputMessageContentText::new(
            outcome.to_string(),
        )),
    )
    .description(format!("Roll to alter reality"));

    let results = vec![InlineQueryResult::Article(result)];

    bot.answer_inline_query(q.id, results)
        .cache_time(0)
        .await?;

    Ok(())
}

pub fn create_handler() -> UpdateHandler<RequestError> {
    Update::filter_inline_query().branch(dptree::endpoint(inline_query_handler))
}
