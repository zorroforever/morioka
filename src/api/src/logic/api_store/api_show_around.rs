use serde_json::json;
use morioka_service::Query;
use morioka_service::sea_orm::DatabaseConnection;

pub async fn handle(
    db: &DatabaseConnection,
    option: Option<serde_json::Value>,
) -> String {
    let json_str: serde_json::Value = option.expect("wrong param!");
    let direction = json_str["direction"].as_str().unwrap().to_string();
    let account_id = json_str["aid"].as_i64().unwrap();
    let character_id = json_str["cid"].as_i64().unwrap();
    let mut map_id = 0;
    let mut position_x = 0;
    let mut position_y = 0;
    let mut position_z = 0;
    // get the character's position
    let character_status = Query::get_character_status_by_character_id(db, character_id as i32).await.unwrap();
    if let Some(v)=character_status {
         map_id = v.map_id.unwrap_or(0);
         position_x = v.x.unwrap_or(0);
         position_y = v.y.unwrap_or(0);
         position_z = v.z.unwrap_or(0);
    }

    let mut pa_x = position_x;
    let mut pa_y = position_y;
    let pa_z = position_z;
    // calculate the position of the character in the direction
    match direction.as_str() {
                "up" => {
                    pa_y =pa_y-1;
                }
                "down" => {
                    pa_y =pa_y+1;
                }
                "left" => {
                    pa_x =pa_x-1;
                }
                "right" => {
                    pa_x =pa_x+1;
                }
               _ => {}
    }
    // get map data at the position
    let map_data = Query::get_map_detail_by_position(db, map_id, pa_x, pa_y, pa_z).await.unwrap();
    if let Some(v)=map_data {
        return json!({ "x": v.x , "y": v.y , "z": v.z , "mid": v.mid , "obj_id":v.obj_id,"acc_able":v.acc_able}).to_string();
    }   else {
        return json!({ "x": 0 , "y": 0 , "z": 0 , "mid": 0 , "obj_id":0,"acc_able":false}).to_string();
    }
}