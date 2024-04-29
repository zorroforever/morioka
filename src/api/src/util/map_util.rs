use morioka_service::Query;
use morioka_service::Excute;
use morioka_service::sea_orm::DatabaseConnection;

pub struct MoriokaPosition{
    pub(crate) x:i32,
    pub(crate) y:i32,
    pub(crate) z:i32,
}

pub struct MoriokaMapDetail{
    position:MoriokaPosition,
    object_id:i32,
    acc_able:i32,
}
pub struct MoriokaMap{
    mid:i32,
    m_width:i32,
    m_height:i32,
    m_length:i32,
    desc:String,
    detail: Vec<MoriokaMapDetail>,
}


pub async fn load_map(
    db: &DatabaseConnection,
    mid:i32,
) -> MoriokaMap{
    let map = Query::load_map_by_mid(
        db,
        mid,
    ).await.unwrap();
    let map_detail = Query::load_map_detail_by_mid(
        db,
        mid,
    ).await.unwrap();
    // 循环map_detail将数据组装成Vec<MoriokaMapDetail>
    let mut map_detail_vec = Vec::new();
    for detail in map_detail{
        let position = MoriokaPosition{
            x:detail.x.unwrap_or(0),
            y:detail.y.unwrap_or(0),
            z:detail.z.unwrap_or(0),
        };
        let map_detail = MoriokaMapDetail{
            position,
            object_id:detail.obj_id.unwrap_or(0),
            acc_able:detail.acc_able.unwrap_or(0),
        };
        map_detail_vec.push(map_detail);
    }
    let res = MoriokaMap{
        mid,
        m_width:map.width.unwrap_or(0),
        m_height:map.height.unwrap_or(0),
        m_length:map.length.unwrap_or(0),
        desc:map.desc.unwrap_or("".to_string()),
        detail:map_detail_vec,
    };
    return res;
}

pub async fn set_character_position_on_map(
    db: &DatabaseConnection,
    character_id:i32,
    mid:i32,
    position:MoriokaPosition,
) -> (){
    Excute::update_character_position_on_map(
        db,
        mid,
        character_id,
        position.x,
        position.y,
        position.z,
    ).await.unwrap();
}