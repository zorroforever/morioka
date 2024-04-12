use rand::Rng;
use chrono::{Datelike, Local, Timelike};
enum Gua {
    Qian,
    Dui,
    Li,
    Zhen,
    Xun,
    Kan,
    Gen,
    Kun,
}
impl Gua {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Gua::Qian,
            1 => Gua::Dui,
            2 => Gua::Li,
            3 => Gua::Zhen,
            4 => Gua::Xun,
            5 => Gua::Kan,
            6 => Gua::Gen,
            _ => Gua::Kun,
        }
    }
}

pub async fn make_gua(

)->Result<Gua,Box<dyn std::error::Error + Send + Sync>>{
    let mut rnd_me = rand::thread_rng();
    let gua_index = rnd_me.gen_range(0..8);
    let gua = Gua::from_index(gua_index);
    Ok(gua)
}
pub struct  Bazi{
    pub year_tiangan: i32,
    pub year_dizhi:i32,
    pub month_tiangan:i32,
    pub month_dizhi:i32,
    pub day_tiangan:i32,
    pub day_dizhi:i32,
    pub hour_tiangan:i32,
    pub hour_dizhi:i32,
}
static TIANGAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
static DIZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

pub async fn calculate_bazi(
) -> Result<Bazi,Box<dyn std::error::Error + Send + Sync>> {
    let local_time = Local::now();
    let year = local_time.year();
    let month = local_time.month();
    let day = local_time.day();
    let hour = local_time.hour();

    let year_tiangan_index = (year - 4) % 10;
    let year_dizhi_index = (year - 4) % 12;
    let year_bazi = format!("{}{}", TIANGAN[year_tiangan_index as usize], DIZHI[year_dizhi_index as usize]);

    let month_tiangan_index = (year_tiangan_index * 2 + month as i32 - 2) % 10;
    let month_bazi = format!("{}{}", TIANGAN[month_tiangan_index as usize], DIZHI[month as usize - 1]);

    let day_tiangan_index = ((year_tiangan_index * 5 + month as i32 + day as i32 - 2) % 10 + 10) % 10;
    let day_bazi = format!("{}{}", TIANGAN[day_tiangan_index as usize], DIZHI[(day - 1) as usize]);

    let hour_tiangan_index = ((day_tiangan_index * 2 + hour as i32 / 2) % 10 + 10) % 10;
    let hour_bazi = format!("{}{}", TIANGAN[hour_tiangan_index as usize], DIZHI[hour as usize / 2]);

    let res = Bazi{
        year_tiangan:year_tiangan_index,
        year_dizhi:year_dizhi_index,
        month_tiangan:year_tiangan_index,
        month_dizhi:year_dizhi_index,
        day_tiangan:year_tiangan_index,
        day_dizhi:year_dizhi_index,
        hour_tiangan:year_tiangan_index,
        hour_dizhi:year_dizhi_index,
    };
   Ok(res)
}

