use crate::{Context, Error};
use crate::services::libcoin::{get_libcoin_balance, deduct_libcoin, grant_libcoin};
use poise::CreateReply;

#[poise::command(
    slash_command,
    description_localized("en-US", "Display your current Libcoin balance."),
    description_localized("fr", "Affiche votre solde actuel de Libcoin"),
)]
pub async fn balance(ctx: Context<'_>) -> Result<(), Error> {
    let user_id: u64 = ctx.author().id.get();
    
    let balance = get_libcoin_balance(user_id)
        .await
        .map_err(|e| Error::from(format!("Failed to get libcoin balance: {}", e)))?;

    ctx.send(CreateReply {
        content: format!("Your current libcoin balance is: **{}**", balance).into(),
        ..Default::default()
    }).await?;
    Ok(())
}