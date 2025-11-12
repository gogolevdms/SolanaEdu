//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove reaction functionality for the Twitter program
///
/// Requirements:
/// - Verify that the tweet reaction exists and belongs to the reaction author
/// - Decrement the appropriate counter (likes or dislikes) on the tweet
/// - Close the tweet reaction account and return rent to reaction author
///
///-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &ctx.accounts.tweet_reaction;

    match tweet_reaction.reaction {
        ReactionType::Like => {
            if tweet.likes == 0 {
                return Err(TwitterError::MinLikesReached.into());
            }
            tweet.likes = tweet
                .likes
                .checked_sub(1)
                .ok_or(TwitterError::MinLikesReached)?;
        }
        ReactionType::Dislike => {
            if tweet.dislikes == 0 {
                return Err(TwitterError::MinDislikesReached.into());
            }
            tweet.dislikes = tweet
                .dislikes
                .checked_sub(1)
                .ok_or(TwitterError::MinDislikesReached)?;
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        mut,
        close = reaction_author,
        has_one = reaction_author,
        constraint = tweet_reaction.parent_tweet == tweet.key()
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
