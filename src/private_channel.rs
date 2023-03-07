use serenity::{prelude::*, model::{prelude::*, Permissions}};

const PRIVATE_CATEGORY: &str = "Privé";

#[derive(Debug)]
pub enum CreatePrivateChannelError {
    NoGuild,
    ImpossibleToCreateChannel(serenity::Error),
    ImpossibleToCreateMessage(serenity::Error),
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

    let user_display_name = user.name.clone() + "-" + &format!("{:04}", user.discriminator);

    // Get or create a category
    // Check if the category exists
    let channels = guild.channels(&ctx.http).await.unwrap();
    let category = channels.iter().find(|c| c.1.name == PRIVATE_CATEGORY);
    let category = if let Some((_, category)) = category {
        category.clone()
    } else {
        let category = guild.create_channel(&ctx.http, |c| {
            c.name(PRIVATE_CATEGORY)
            .kind(ChannelType::Category)
        }).await.unwrap();
        category.clone()
    };
    // Create a private text channel
    let channel = guild.create_channel(&ctx.http, |c| {
        let permissions = vec![
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(RoleId(guild.id.0)),
            },
            PermissionOverwrite {
                allow: Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(UserId(user.id.0)),
            },
            PermissionOverwrite {
                allow: Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(ctx.cache.current_user_id()),       
            },
        ];
        c.name(format!("{user_display_name}-private"))
                .category(category.id)
                .kind(ChannelType::Text)
                .permissions(permissions)
        
    }).await;



    let channel = match channel {
        Ok(channel) => channel,
        Err(e) => return Err(ImpossibleToCreateChannel(e)),
    };
    
    // Send a message to the user
    channel.say(&ctx.http, format!("Salut {user_name} !", user_name = user.name)).await.map_err(ImpossibleToCreateMessage)?;
    

    Ok(())
}