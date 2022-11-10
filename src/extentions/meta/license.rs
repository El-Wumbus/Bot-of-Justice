use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};

pub struct License;

impl License {
    pub const GPLV2:&'static str =
        "Summary:\n You may copy, distribute and modify the software as long\"
        as you track changes/dates in source files.\"
        Any modifications to or software including (via compiler)\"
        GPL-licensed code must also be made available under the GPL\"
        along with build & install instructions.\n\"
        The license document can be found at \"
        'https://github.com/el-wumbus/Bot-of-Justice/blob/master/LICENSE'";

    pub const MIT:&'static str =
        "Basically, you can do whatever you want as long as you include\"
        the original copyright and license notice in any copy of the software/source.\"
        Full License document: https://tldrlegal.com/license/mit-license#fulltext";
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("linfo")
        .description("Print info about software licenses")
        .create_option(|option|
        {
            option
                .name("license")
                .description("The license to print info about")
                .kind(CommandOptionType::String)
                .required(true)
        })
}