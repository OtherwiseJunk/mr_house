use crate::{Context, Error};
use crate::services::libcoin::{get_libcoin_balance, deduct_libcoin, grant_libcoin};
use poise::CreateReply;

#[poise::command(slash_command)]
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

#[poise::command(slash_command)]
pub async fn deduct(ctx: Context<'_>, amount: f64) -> Result<(), Error> {
    let user_id: u64 = ctx.author().id.get();

    if user_id != 94545463906144256{
        return Err(Error::from("You are not authorized to use this command."));
    }

    deduct_libcoin(user_id, amount, "Deducted by Mr_House Libcoin Check Command")
        .await
        .map_err(|e| Error::from(format!("Failed to deduct libcoin: {}", e)))?;

    ctx.send(CreateReply {
        content: format!("Successfully deducted **{}** libcoin.", amount).into(),
        ..Default::default()
    }).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn grant(ctx: Context<'_>, amount: f64) -> Result<(), Error> {
    let user_id: u64 = ctx.author().id.get();

    if user_id != 94545463906144256{
        return Err(Error::from("You are not authorized to use this command."));
    }

    grant_libcoin(user_id, amount, "Deducted by Mr_House Libcoin Check Command")
        .await
        .map_err(|e| Error::from(format!("Failed to deduct libcoin: {}", e)))?;

    ctx.send(CreateReply {
        content: format!("Successfully deducted **{}** libcoin.", amount).into(),
        ..Default::default()
    }).await?;
    Ok(())
}