use chrono::{DateTime, NaiveDateTime, Utc};
use teloxide::{
    payloads::{KickChatMemberSetters, SendMessageSetters},
    prelude::{GetChatId, Requester},
    types::ParseMode,
    utils::html::{self, user_mention_or_link},
};

use crate::{
    database::db_utils::get_log_channel,
    get_mdb,
    modules::send_log,
    util::{
        check_command_disabled, extract_text_id_from_reply, get_bot_id, get_chat_title, get_time,
        is_group, sudo_or_owner_filter, user_should_restrict, TimeUnit,
    },
    Cxt, TgErr, OWNER_ID, SUDO_USERS,
};

pub async fn ban(cx: &Cxt) -> TgErr<()> {
    tokio::try_join!(
        is_group(cx),                                           //Should be a group
        user_should_restrict(cx, get_bot_id(cx).await),         //Bot Should have restrict rights
        user_should_restrict(cx, cx.update.from().unwrap().id), //User should have restrict rights
    )?;
    let db = get_mdb().await;
    let bot_id = get_bot_id(cx).await;
    let (user_id, text) = extract_text_id_from_reply(cx).await;
    let reason = text.unwrap_or_else(|| String::from("None"));
    if user_id.is_none() {
        cx.reply_to("请以/ban回复封禁对象发出的消息来执行指令").await?;
        return Ok(());
    }
    if user_id.unwrap() == bot_id {
        cx.reply_to("#查询精神状态")
            .await?;
        return Ok(());
    }

    if user_id.unwrap() == *OWNER_ID || (*SUDO_USERS).contains(&user_id.unwrap()) {
        cx.reply_to("失败了失败了失败了失败了失败了")
            .await?;
        return Ok(());
    }
    if let Ok(mem) = cx
        .requester
        .get_chat_member(cx.chat_id(), user_id.unwrap())
        .await
    {
        if mem.is_banned() {
            cx.reply_to("此人已被封禁，无需重复操作").await?;
            return Ok(());
        }
     //   if mem.user_is_admin() {
       //     cx.reply_to("你先想办法把他管理员撤了").await?;
      //      return Ok(());
    //    }
    } else {
        cx.reply_to("获取封禁对象信息失败，请尝试直接回复封禁对象发出的原始消息而非转发后回复")
            .await?;
        return Ok(());
    };
    let user = cx
        .requester
        .get_chat_member(cx.chat_id(), user_id.unwrap())
        .await
        .unwrap()
        .user;
    let ban_text = format!(
        "已封禁{}，原因（如有）：{}",
        user_mention_or_link(&user),
        reason
    );
    cx.requester
        .kick_chat_member(cx.chat_id(), user_id.unwrap())
        .await?;
    cx.reply_to(ban_text).parse_mode(ParseMode::Html).await?;

    if let Some(l) = get_log_channel(&db, cx.chat_id()).await? {
        let admin = cx
            .requester
            .get_chat_member(cx.chat_id(), cx.update.from().unwrap().id)
            .await?
            .user;
        let user = cx
            .requester
            .get_chat_member(cx.chat_id(), user_id.unwrap())
            .await?
            .user;
        let logm = format!(
            "Chat Title: {}\n#BANNED\nAdmin: {}\nUser: {}",
            html::code_inline(&get_chat_title(cx, cx.chat_id()).await.unwrap()),
            html::user_mention(admin.id, &admin.full_name()),
            html::user_mention(user_id.unwrap(), &user.full_name())
        );
        send_log(cx, &logm, l).await?;
    }
    Ok(())
}

/* pub async fn temp_ban(cx: &Cxt) -> TgErr<()> {
    tokio::try_join!(
        is_group(cx),                                           //Should be a group
        user_should_restrict(cx, get_bot_id(cx).await),         //Bot Should have restrict rights
        user_should_restrict(cx, cx.update.from().unwrap().id), //User should have restrict rights
    )?;
    let (user_id, text) = extract_text_id_from_reply(cx).await;
    let bot_id = get_bot_id(cx).await;
    let db = get_mdb().await;
    if user_id.is_none() {
        cx.reply_to("请以/tban回复封禁对象发出的消息来执行指令").await?;
        return Ok(());
    }

    if text.is_none() {
        cx.reply_to("请指定临时封禁时间，可使用s（秒）m（分）h（时）d（天），永久封禁请直接用/ban").await?;
        return Ok(());
    }

    if user_id.unwrap() == bot_id {
        cx.reply_to("#查询精神状态")
            .await?;
        return Ok(());
    }
    if user_id.unwrap() == *OWNER_ID || (*SUDO_USERS).contains(&user_id.unwrap()) {
        cx.reply_to("失败了失败了失败了失败了失败了")
            .await?;
        return Ok(());
    }

    if let Ok(mem) = cx
        .requester
        .get_chat_member(cx.chat_id(), user_id.unwrap())
        .await
    {
    //    if !mem.can_be_edited() {
        //    cx.reply_to("你先想办法把他管理员撤了").await?;
      //      return Ok(());
    //    }

        if mem.is_banned() {
            cx.reply_to("此人已被封禁，无需重复操作").await?;
            return Ok(());
        }

        if sudo_or_owner_filter(user_id.unwrap()).await.is_ok() {
            cx.reply_to("失败了失败了失败了失败了失败了")
                .await?;
            return Ok(());
        }

        if user_id.unwrap() == get_bot_id(cx).await {
            cx.reply_to("#查询精神状态").await?;
            return Ok(());
        }
        let u = text.unwrap().parse::<TimeUnit>();
        if u.is_err() {
            cx.reply_to("请指定临时封禁时间，可使用s（秒）m（分）h（时）d（天），永久封禁请直接用/ban")
                .await?;
            return Ok(());
        }
        let t = get_time(u.as_ref().unwrap());
        cx.requester
            .kick_chat_member(cx.chat_id(), user_id.unwrap())
            .until_date(
                DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(cx.update.date as i64, 0),
                    Utc,
                ) + t,
            )
            .await?;
        cx.reply_to(format!("<b>已封禁<i>{}</i></b> ", u.as_ref().unwrap()))
            .parse_mode(ParseMode::Html)
            .await?;
        if let Some(l) = get_log_channel(&db, cx.chat_id()).await? {
            let admin = cx
                .requester
                .get_chat_member(cx.chat_id(), cx.update.from().unwrap().id)
                .await?
                .user;
            let mem = cx
                .requester
                .get_chat_member(cx.chat_id(), user_id.unwrap())
                .await?;
            let logm = format!(
                "Chat title: {}\n#TEMP_BANNED\n操作者: {}\n被封禁者: {}\n 解除时间: {}\n",
                html::code_inline(&get_chat_title(cx, cx.chat_id()).await.unwrap()),
                html::user_mention(admin.id, &admin.full_name()),
                html::user_mention(user_id.unwrap(), &mem.user.full_name()),
                html::code_inline(&mem.until_date().unwrap().to_string())
            );
            send_log(cx, &logm, l).await?;
        }
    } else {
        cx.reply_to("获取封禁对象信息失败，请尝试直接回复封禁对象发出的原始消息而非转发后回复")
            .await?;
    }

    Ok(())
} */

pub async fn unban(cx: &Cxt) -> TgErr<()> {
    tokio::try_join!(
        is_group(cx),                                           //Should be a group
        user_should_restrict(cx, get_bot_id(cx).await),         //Bot Should have restrict rights
        user_should_restrict(cx, cx.update.from().unwrap().id), //User should have restrict rights
    )?;
    let db = get_mdb().await;
    let (user_id, _text) = extract_text_id_from_reply(cx).await;
    if user_id.is_none() {
        cx.reply_to("请以/unban回复解封对象发出的消息来执行指令").await?;
        return Ok(());
    }

    if let Ok(mem) = cx
        .requester
        .get_chat_member(cx.chat_id(), user_id.unwrap())
        .await
    {
        if !mem.is_banned() {
            cx.reply_to("然而他本来就没被封禁").await?;
            return Ok(());
        }
    } else {
        cx.reply_to("获取解封对象信息失败，请尝试直接回复解封对象发出的原始消息而非转发后回复")
            .await?;
        return Ok(());
    }

    cx.requester
        .unban_chat_member(cx.chat_id(), user_id.unwrap())
        .await?;
    cx.reply_to("已解封")
        .parse_mode(ParseMode::Html)
        .await?;

    if let Some(l) = get_log_channel(&db, cx.chat_id()).await? {
        let admin = cx
            .requester
            .get_chat_member(cx.chat_id(), cx.update.from().unwrap().id)
            .await?
            .user;
        let user = cx
            .requester
            .get_chat_member(cx.chat_id(), user_id.unwrap())
            .await?
            .user;
        let logm = format!(
            "Chat title: {}\n#UNBANNED\nAdmin: {}\nUser: {}",
            html::code_inline(&get_chat_title(cx, cx.chat_id()).await.unwrap()),
            html::user_mention(admin.id, &admin.full_name()),
            html::user_mention(user_id.unwrap(), &user.full_name())
        );
        send_log(cx, &logm, l).await?;
    }
    Ok(())
}

pub async fn kick(cx: &Cxt) -> TgErr<()> {
    tokio::try_join!(
        is_group(cx),                                           //Should be a group
        user_should_restrict(cx, get_bot_id(cx).await),         //Bot Should have restrict rights
        user_should_restrict(cx, cx.update.from().unwrap().id), //User should have restrict rights
    )?;
    let db = get_mdb().await;
    let bot_id = get_bot_id(cx).await;
    let (user_id, text) = extract_text_id_from_reply(cx).await;
    if user_id.is_none() {
        cx.reply_to("请以/kick回复踢出对象发出的消息来执行指令").await?;
        return Ok(());
    }
    if user_id.unwrap() == bot_id {
        cx.reply_to("#查询精神状态")
            .await?;
        return Ok(());
    }

    if user_id.unwrap() == *OWNER_ID || (*SUDO_USERS).contains(&user_id.unwrap()) {
        cx.reply_to("失败了失败了失败了失败了失败了")
            .await?;
        return Ok(());
    }
    if let Ok(mem) = cx
        .requester
        .get_chat_member(cx.chat_id(), user_id.unwrap())
        .await
    {
        if mem.is_banned() || mem.is_left() {
            cx.reply_to("他已经不在群里了，如果要阻止他回来请用/ban").await?;
            return Ok(());
        }
 //       if !mem.can_be_edited() {
    //        cx.reply_to("你先想办法把他管理员撤了").await?;
  //          return Ok(());
     //   }
    } else {
        cx.reply_to("获取封禁对象信息失败，请尝试直接回复封禁对象发出的原始消息而非转发后回复")
            .await?;
        return Ok(());
    };
    let user = cx
        .requester
        .get_chat_member(cx.chat_id(), user_id.unwrap())
        .await
        .unwrap()
        .user;
    let reason = text.unwrap_or_else(|| String::from("None"));
    let kick_text = format!(
        "已踢出{}，原因（如有）：{}",
        user_mention_or_link(&user),
        reason
    );
    cx.requester
        .kick_chat_member(cx.chat_id(), user_id.unwrap())
        .await?;
    cx.requester
        .unban_chat_member(cx.chat_id(), user_id.unwrap())
        .await?;
    cx.reply_to(kick_text).parse_mode(ParseMode::Html).await?;
    if let Some(l) = get_log_channel(&db, cx.chat_id()).await? {
        let admin = cx
            .requester
            .get_chat_member(cx.chat_id(), cx.update.from().unwrap().id)
            .await?
            .user;
        let user = cx
            .requester
            .get_chat_member(cx.chat_id(), user_id.unwrap())
            .await?
            .user;
        let logm = format!(
            "Chat title: {}\n#KICKED\nAdmin: {}\nUser: {}",
            html::code_inline(&get_chat_title(cx, cx.chat_id()).await.unwrap()),
            html::user_mention(admin.id, &admin.full_name()),
            html::user_mention(user_id.unwrap(), &user.full_name())
        );
        send_log(cx, &logm, l).await?;
    }
    Ok(())
}
pub async fn kickme(cx: &Cxt, cmd: &str) -> TgErr<()> {
    tokio::try_join!(
        is_group(cx),
        user_should_restrict(cx, get_bot_id(cx).await),
        check_command_disabled(cx, String::from(cmd))
    )?;
    let db = get_mdb().await;
    if let Some(user) = cx.update.from() {
        let user_id = user.id;
        if user_id == *OWNER_ID || (*SUDO_USERS).contains(&user_id) {
            cx.reply_to("您还是自己退群吧")
                .await?;
            return Ok(());
        }
        if let Ok(mem) = cx.requester.get_chat_member(cx.chat_id(), user_id).await {
      //      if !mem.can_be_edited() {
      //          cx.reply_to("你先想办法把他管理员撤了").await?;
     //           return Ok(());
   //         }
        } else {
            cx.reply_to("Can't kick the user").await?;
            return Ok(());
        }
        let kickme_text = format!("如你所愿，{}再见", user_mention_or_link(user));
        cx.requester.kick_chat_member(cx.chat_id(), user_id).await?;
        cx.requester
            .unban_chat_member(cx.chat_id(), user_id)
            .await?;
        cx.reply_to(kickme_text).await?;
        if let Some(l) = get_log_channel(&db, cx.chat_id()).await? {
            let user = cx
                .requester
                .get_chat_member(cx.chat_id(), user_id)
                .await?
                .user;
            let logm = format!(
                "Chat id: {}\n#KICKME\nUser: {}",
                html::code_inline(&get_chat_title(cx, cx.chat_id()).await.unwrap()),
                html::user_mention(user_id, &user.full_name())
            );
            send_log(cx, &logm, l).await?;
        }
    } else {
        cx.reply_to("获取封禁对象信息失败，请尝试直接回复封禁对象发出的原始消息而非转发后回复").await?;
    }
    Ok(())
}
