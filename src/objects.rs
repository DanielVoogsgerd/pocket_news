

struct MessageCollection {
    size: u32,
    articles: VecDeque<Article>,
}


struct Article {
    title: String,
    url: String,
    identifier: String,
}
