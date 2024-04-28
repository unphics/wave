use std::collections::HashMap;
use wave::entity;

enum svr_type {
    Base,
}
struct server {
    svr_type: svr_type,
    map_entitys: HashMap<i32, i32>, // TODO map的类型改为<id, entity>

}