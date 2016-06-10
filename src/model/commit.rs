
/// Format:
/// "commit "[length]\0
/// "tree "[tree hash in hex ascii]\0
/// "parent "[parent commit hash in hex ascii]\0 (repeat for each parent)\0x0A
/// "author "[author name in utf8]" <"[author email]"> "[unix timestamp]" "[timezone]\0x0A
/// "commiter "[commiter name in utf8]" <"[author email]"> "[unix timestamp]" "[timezone]\0x0A
/// \0x0A
/// [commit message]
/// TODO: gpg signing? other infos
pub struct Commit {
    pub tree: Rc<Tree>,
    pub parents: Vec<Rc<Commit>>,
    pub author: (Contact, TimeInfo),
    pub committer: (Contact, TimeInfo)
}
