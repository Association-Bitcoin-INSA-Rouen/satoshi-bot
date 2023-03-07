use serenity::{prelude::*, model::{prelude::*, Permissions}};


pub enum CreatePrivateChannelError {
    NoGuild
}

pub async fn create_private_channel(ctx: &Context, add_reaction: &Reaction) -> Result<(), CreatePrivateChannelError> {
    use CreatePrivateChannelError::*;

    // Get the guild id
    let guild = if let Some(guid) = add_reaction.guild_id {
        guid
    } else {
        return Err(NoGuild);
    };

    // Create channel group 
    let guild = guild.to_partial_guild(&ctx.http).await.unwrap();

    // Get the user
    let user = add_reaction.user(&ctx.http).await.unwrap();

    let user_display_name = user.name.clone() + "-" + &user.discriminator.to_string();

    // Get or create a category
    // Check if the category exists
    let channels = guild.channels(&ctx.http).await.unwrap();
    let category = channels.iter().find(|c| c.1.name == "Private Channels");
    let category = if let Some((_, category)) = category {
        category.clone()
    } else {
        let category = guild.create_channel(&ctx.http, |c| {
            c.name("Private Channels")
            .kind(ChannelType::Category)
        }).await.unwrap();
        category.clone()
    };




    let channel = guild.create_channel(&ctx.http, |c| {
        c.name(format!("{user_display_name}-private"))
        .category(category.id)
        .kind(ChannelType::Text)
        
    }).await.unwrap();

    // Add the user to the channel and make it private
    // Create the permission overwrite

   

    Ok(())
}