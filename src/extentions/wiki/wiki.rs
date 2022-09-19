use wikipedia;

pub fn run(search_term: String, id: bool) -> String
{
    return wiki_summary(get_wiki_id(search_term, id));
}

fn wiki_summary(id: String) -> String
{
    let handle = wikipedia::Wikipedia::<wikipedia::http::hyper::Client>::default();
    let page = handle.page_from_pageid(id);

    let content = match page.get_summary()
    {
        Ok(x) => x,
        Err(x) => format!("Error: {}", x),
    };

    if !content.len() >= 2000
    {
        return content;
    }

    match page.get_title()
    {
        Ok(x) =>
        {
            format!("Error: Wiki Summary surpasses discord's 2000 character limit.\nhttps://en.wikipedia.org/wiki/{}",x.trim().replace(" ", "_"))
        }
        Err(x) => format!("Error: {}", x),
    }
}

fn get_wiki_id(search_term: String, id: bool) -> String
{
    return match id
    {
        true => search_term,
        false =>
        {
            let pageid = match wikipedia::Wikipedia::<wikipedia::http::hyper::Client>::default()
                .page_from_title(search_term)
                .get_pageid()
            {
                Ok(x) => x,
                Err(_) => "".to_string(),
            };
            pageid
        }
    };
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn get_wiki_id_search_test()
    {
        assert_eq!(get_wiki_id("Linux".to_string(), false), "6097297");
    }

    #[test]
    fn get_wiki_id_id_test()
    {
        assert_eq!(get_wiki_id("6097297".to_string(), true), "6097297");
    }

    #[test]
    fn run_id_test()
    {
        assert_eq!(run("6097297".to_string(), true), "Linux ( (listen) LEE-nuuks or  LIN-uuks) is an open-source Unix-like operating system based on the Linux kernel, an operating system kernel first released on September 17, 1991, by Linus Torvalds. Linux is typically packaged as a Linux distribution.\nDistributions include the Linux kernel and supporting system software and libraries, many of which are provided by the GNU Project. Many Linux distributions use the word \"Linux\" in their name, but the Free Software Foundation uses the name \"GNU/Linux\" to emphasize the importance of GNU software, causing some controversy.Popular Linux distributions include Debian, Fedora Linux, and Ubuntu, which in itself has many different distributions and modifications, including Lubuntu and Xubuntu. Commercial distributions include Red Hat Enterprise Linux and SUSE Linux Enterprise. Desktop Linux distributions include a windowing system such as X11 or Wayland, and a desktop environment such as GNOME or KDE Plasma. Distributions intended for servers may omit graphics altogether, or include a solution stack such as LAMP. Because Linux is freely redistributable, anyone may create a distribution for any purpose.Linux was originally developed for personal computers based on the Intel x86 architecture, but has since been ported to more platforms than any other operating system. Because of the dominance of the Linux-based Android on smartphones, Linux, including Android, has the largest installed base of all general-purpose operating systems, as of May 2022. Although Linux is, as of May 2022, used by only around 2.3 percent of desktop computers, the Chromebook, which runs the Linux kernel-based Chrome OS, dominates the US K–12 education market and represents nearly 20 percent of sub-$300 notebook sales in the US. Linux is the leading operating system on servers (over 96.4% of the top 1 million web servers' operating systems are Linux), leads other big iron systems such as mainframe computers, and is the only OS used on TOP500 supercomputers (since November 2017, having gradually eliminated all competitors).Linux also runs on embedded systems, i.e. devices whose operating system is typically built into the firmware and is highly tailored to the system. This includes routers, automation controls, smart home devices, IP Cameras, video game consoles, televisions (Samsung and LG Smart TVs use Tizen and WebOS, respectively), automobiles (Tesla, Audi, Mercedes-Benz, Hyundai, and Toyota all rely on Linux), Spacecraft (Falcon 9's and Dragon 2's avionics use a customized version of Linux), and Rovers (The Mars 2020 Mission carried 3 Linux computers to Mars including the Ingenuity Helicopter).\nLinux is one of the most prominent examples of free and open-source software collaboration. The source code may be used, modified and distributed commercially or non-commercially by anyone under the terms of its respective licenses, such as the GNU General Public License (GPL). The Linux kernel, for example, is licensed under the GPLv2, with a special exception for system calls, as without the system call exception any program calling on the kernel would be considered a derivative and therefore the GPL would have to apply to that program.");
    }

    #[test]
    fn run_search_test()
    {
        assert_eq!(run("Linux".to_string(), false), "Linux ( (listen) LEE-nuuks or  LIN-uuks) is an open-source Unix-like operating system based on the Linux kernel, an operating system kernel first released on September 17, 1991, by Linus Torvalds. Linux is typically packaged as a Linux distribution.\nDistributions include the Linux kernel and supporting system software and libraries, many of which are provided by the GNU Project. Many Linux distributions use the word \"Linux\" in their name, but the Free Software Foundation uses the name \"GNU/Linux\" to emphasize the importance of GNU software, causing some controversy.Popular Linux distributions include Debian, Fedora Linux, and Ubuntu, which in itself has many different distributions and modifications, including Lubuntu and Xubuntu. Commercial distributions include Red Hat Enterprise Linux and SUSE Linux Enterprise. Desktop Linux distributions include a windowing system such as X11 or Wayland, and a desktop environment such as GNOME or KDE Plasma. Distributions intended for servers may omit graphics altogether, or include a solution stack such as LAMP. Because Linux is freely redistributable, anyone may create a distribution for any purpose.Linux was originally developed for personal computers based on the Intel x86 architecture, but has since been ported to more platforms than any other operating system. Because of the dominance of the Linux-based Android on smartphones, Linux, including Android, has the largest installed base of all general-purpose operating systems, as of May 2022. Although Linux is, as of May 2022, used by only around 2.3 percent of desktop computers, the Chromebook, which runs the Linux kernel-based Chrome OS, dominates the US K–12 education market and represents nearly 20 percent of sub-$300 notebook sales in the US. Linux is the leading operating system on servers (over 96.4% of the top 1 million web servers' operating systems are Linux), leads other big iron systems such as mainframe computers, and is the only OS used on TOP500 supercomputers (since November 2017, having gradually eliminated all competitors).Linux also runs on embedded systems, i.e. devices whose operating system is typically built into the firmware and is highly tailored to the system. This includes routers, automation controls, smart home devices, IP Cameras, video game consoles, televisions (Samsung and LG Smart TVs use Tizen and WebOS, respectively), automobiles (Tesla, Audi, Mercedes-Benz, Hyundai, and Toyota all rely on Linux), Spacecraft (Falcon 9's and Dragon 2's avionics use a customized version of Linux), and Rovers (The Mars 2020 Mission carried 3 Linux computers to Mars including the Ingenuity Helicopter).\nLinux is one of the most prominent examples of free and open-source software collaboration. The source code may be used, modified and distributed commercially or non-commercially by anyone under the terms of its respective licenses, such as the GNU General Public License (GPL). The Linux kernel, for example, is licensed under the GPLv2, with a special exception for system calls, as without the system call exception any program calling on the kernel would be considered a derivative and therefore the GPL would have to apply to that program.");
    }
}
