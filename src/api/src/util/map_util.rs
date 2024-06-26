use serde::{Deserialize, Serialize};
use morioka_service::Query;
use morioka_service::Excute;
use morioka_service::sea_orm::{DatabaseConnection, DeriveEntityModel};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoriokaPosition {
    pub(crate) x:i32,
    pub(crate) y:i32,
    pub(crate) z:i32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoriokaMapDetail{
    position:MoriokaPosition,
    object_id:i32,
    acc_able:i32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoriokaMap{
    pub(crate) mid:i32,
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

pub async fn get_map_position_by_character_id(
    db: &DatabaseConnection,
    character_id:i32,
) -> (i32,MoriokaPosition){
    let character_status = Query::get_character_status_by_character_id(
        db,
        character_id,
    ).await.unwrap();

    let res = MoriokaPosition{
        x:character_status.x.unwrap_or(0),
        y:character_status.y.unwrap_or(0),
        z:character_status.z.unwrap_or(0),
    };
     (character_status.map_id.unwrap_or(0),res)
}

pub async fn get_map_detail_by_position(
    db: &DatabaseConnection,
    mid:i32,
    position:MoriokaPosition,
) -> (MoriokaMapDetail){
    let map_detail = Query::get_map_detail_by_position(
        db,
        mid,
        position.x,
        position.y,
        position.z,
    ).await;
    if let (Ok(_value))    = map_detail {
        let res = MoriokaMapDetail{
            position:MoriokaPosition{
                x:_value.x.unwrap_or(0),
                y:_value.y.unwrap_or(0),
                z:_value.z.unwrap_or(0),
            },
            object_id:_value.obj_id.unwrap_or(0),
            acc_able:_value.acc_able.unwrap_or(0),
        };
        res
    }else {
        MoriokaMapDetail {
            position: MoriokaPosition {
                x: 0,
                y: 0,
                z: 0,
            },
            object_id: 0,
            acc_able: 0,
        }
    }
}