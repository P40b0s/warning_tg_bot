use std::{borrow::Cow, collections::HashMap, fmt::format};

use serde::{Deserialize, Serialize};
use utilites::Date;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupSettings
{
    pub chat_id: i64,
    pub users_count: u32,
    pub deadline_time: Date,
    pub additional_dates: Vec<Date>,
    pub group_name: Option<String>,
    pub is_active: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group
{
    chat_id: i64,
    users: Vec<User>,
    settings: Option<GroupSettings>,
}

impl Group
{
    pub fn new(chat_id: i64, users: Vec<User>) -> Self
    {
        Self
        {
            chat_id,
            users,
            settings: None
        }
    }
    pub fn add_settings(self, settings: GroupSettings) -> Self
    {
        Self
        {
            settings : Some(settings),
            ..self
        }
    }
    pub fn get_settings<'a>(&'a self) -> Option<&'a GroupSettings>
    {
        self.settings.as_ref()
    }
    
    pub fn get_process(current_count: usize, overall_count: u32) -> String
    {
        if current_count == 0 || overall_count == 0
        {
            return "🟥".repeat(10);
        }
        let percent: u32 = ((current_count as f32 / overall_count as f32) * 10.0) as u32;
        let red_count = 10 - percent;
        ["🟩".repeat(percent as usize), "🟥".repeat(red_count as usize), (percent*10).to_string(), "%".to_owned()].concat()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Status
{
    ///Ready
    Plus,
    ///Unready
    Minus,
    Disease(String, Date),
    Vacation(Date, Date)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User
{
    pub id: u64,
    pub username: String,
    pub nick: Option<String>,
    pub updated: Date,
    pub current_status: Status
}
impl PartialEq for User
{
    fn eq(&self, other: &Self) -> bool 
    {
        &self.id == &other.id
    }
}
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct State
// {
//     user: User,
//     status: Status
// }
// impl PartialEq for State
// {
//     fn eq(&self, other: &Self) -> bool 
//     {
//         if self.user.is_none() || other.user.is_none()
//         {
//             return false;
//         }
//         let u1 = self.user.as_ref().unwrap();
//         let u2 = other.user.as_ref().unwrap();
//         u1 == u2
//     }
// }

// impl Default for State
// {
//     fn default() -> Self 
//     {
//         Self
//         {
//             user: None,
//             status: Status::Minus
//         }
//     }
// }
impl User
{
    pub fn new(id: u64, username: String, nick: Option<String>, date: Date, current_status: Status) -> Self
    {
        //let date = date.add_minutes(3 * 60);
        Self 
        {
            id,
            username,
            nick,
            updated: date,
            current_status
        }
    }
    pub fn change_status(&mut self, status: Status)
    {
        self.current_status = status;
        self.updated = Date::now();
    }
}
// impl State
// {
//     pub fn new(user: Option<User>) -> Self
//     {
//         if let Some(user) = user
//         {
//             Self
//             {
//                 user: Some(user),
//                 status: Status::Plus
//             }
//         }
//         else 
//         {
//             Self::default()    

//         }
//     }
//     pub fn change_status(&mut self, status: Status)
//     {
//         self.status = status;
//         if let Some(u) = self.user.as_mut()
//         {
//             u.updated = Date::now();
//         }
//     }
// }
impl ToString for Group
{
    fn to_string(&self) -> String 
    {
        let mut output = String::new();
        let plus_count = self.users.iter().filter(|f| f.current_status == Status::Plus).count();
        let settings = self.get_settings();
        let mut count = settings.and_then(|c| Some(c.users_count)).unwrap_or_default();
        let not_ready_count = self.users.iter().filter(|f| f.current_status != Status::Minus && f.current_status != Status::Plus).count();
        output.push_str(&format!("*Статус {}/{}*\n", plus_count, count - not_ready_count as u32));
        output.push_str(&[Self::get_process(plus_count, count - not_ready_count as u32), "\n".to_owned()].concat());
        output.push_str(&divider());
        for i in  0..count as usize
        {
           if let Some(u) = self.users.get(i)
           {
                let nick = match u.nick.as_ref()
                {
                    Some(n) => format!("\\([{}](tg://user?id={})\\)",teloxide::utils::markdown::escape(&n), u.id),
                    None => "".to_owned()
                };
                let date = u.updated.format(utilites::DateFormat::Serialize);
                let date = date.split("T").collect::<Vec<_>>();
                let status_string = match &u.current_status
                {
                    Status::Minus =>  format!("❌ {} {}\n🕛 *{} {}*\n", teloxide::utils::markdown::escape(&u.username), nick, date[0].replace("-", "\\."), date[1]),
                    Status::Plus => format!("✅ {} {}\n🕛 *{} {}*\n", teloxide::utils::markdown::escape(&u.username), nick, date[0].replace("-", "\\."), date[1]),
                    Status::Disease(dia, to) =>
                    {
                        count -=1;
                        format!("🚑 {} {}\n🏥 болен: *{}* до: *{}*\n", teloxide::utils::markdown::escape(&u.username), nick, dia, teloxide::utils::markdown::escape(&to.format(utilites::DateFormat::DotDate)))
                    },
                    Status::Vacation(from, to) => 
                    {
                        count -=1;
                        format!("🍺 {} {}\n🎉 отпуск с *{}* по: *{}*\n", teloxide::utils::markdown::escape(&u.username), nick, teloxide::utils::markdown::escape(&from.format(utilites::DateFormat::DotDate)), teloxide::utils::markdown::escape(&to.format(utilites::DateFormat::DotDate)))
                    }
                };
                output.push_str(&status_string);
                output.push_str(&divider());
           }
           else 
           {
                output.push_str("❌ Неизвестный\n");
                output.push_str(&divider());
           }
        }
        output
    }
    
}


impl ToString for GroupSettings
{
    fn to_string(&self) -> String 
    {
        let mut output = String::new();
        output.push_str("*⚙   Настройки   ⚙*\n");
        output.push_str(&divider());
        output.push_str(&["🐣 Количетво проверяемых: *",self.users_count.to_string().as_str(), "*\n"].concat());
        output.push_str(&divider());
        output.push_str(&["⏳ Отчетное время: *",self.deadline_time.format(utilites::DateFormat::Time).as_str(), "*\n"].concat());
        output.push_str(&divider());
        if self.additional_dates.len() > 0
        {
            output.push_str("*🕡 Дополнительные даты 🕡*\n");
            output.push_str(&divider());
            for d in &self.additional_dates
            {
                output.push_str(&["⏰ *", teloxide::utils::markdown::escape(&d.format(utilites::DateFormat::DotDate)).as_str(), "*\n"].concat());
            }
            output.push_str(&divider());
        }
        output
    }
}


fn divider<'a>() -> Cow<'a, str>
{
    Cow::from(["➖".repeat(14), "\n".to_owned()].concat())
}
// impl User
// {
//     pub fn new(id: u64, name: String, nick: Option<String>, date: Date) -> Self
//     {
//         let date = date.add_minutes(3 * 60);
//         Self 
//         {
//             id,
//             name,
//             nick,
//             updated: date
//         }
//     }
// }

#[cfg(test)]
mod tests
{
    use std::time::Instant;

    use rand::Rng;


    pub fn get_process(current_count: u32, overall_count: u32) -> String
    {
        if current_count == 0
        {
            //return "🟩"
            return "🟥".repeat(10);
        }
        let count = overall_count - current_count;
        let percent: u32 = ((current_count as f32 / overall_count as f32) * 10.0) as u32;
        logger::debug!("{} {}", count,  percent);
        let red_count = 10 - percent;
        ["🟩".repeat(percent as usize), "🟥".repeat(red_count as usize)].concat()
    }
    #[test]
    pub fn test_settings()
    {
        logger::StructLogger::new_default();
        logger::debug!("{}", get_process(10, 10));
    }


    #[test]
    pub fn test_rnd()
    {
        logger::StructLogger::new_default();
        let mut rng = rand::thread_rng();
        let parts = 3;
        let summ = 99;
        let parts_val = summ / parts;
        let mut prices = vec![];
        for _ in 0..parts
        {
            prices.push(parts_val);
        }
        let mut current_percent = rng.gen_range(0.05..=0.16);
        let mut current_d = rng.gen_range(0..=1);
        let prices_len = prices.len();
        for (i, p) in prices.iter_mut().enumerate()
        {
            if i < prices_len
            {
                if i % 2 == 0
                {
                    current_percent = rng.gen_range(0.05..=0.16);
                    let new = ((*p as f32) * current_percent).ceil() as i32;
                    *p = *p + new;
                }   
                else 
                {
                    let new = ((*p as f32) * current_percent).ceil() as i32;
                    *p = *p - new;
                }
            }
        }
        let sum: i32 = prices.iter().sum();
        let minus = sum - summ;
        logger::debug!("sum: {}, source: {}, ,minus: {}, arr: {:?}", sum, summ, minus, &prices);
        *prices.last_mut().unwrap() -= minus;
        // let (a, b) = match &prices[..]
        // {
        //     &[first, second, ..] => (first, second),
        //     _ => unreachable!(),
        // };
        let sum: i32 = prices.iter().sum();
        logger::debug!("{:?}, {}", &prices, sum);
    }

    #[test]
    pub fn test_rnd2()
    {
        logger::StructLogger::new_default();
        let now = Instant::now();
        let n1 = 0;
        //let p = split_orders_by_total_sum(1000, 456547894, 1, 15, 30);
        //let n1 =  now.elapsed().as_secs();
        let p = split_orders_by_total_sum_2(1000, 456547894.0);
        let n2 =  now.elapsed().as_secs();
        logger::error!("{} {}", n1, n2);
    }
    

    fn get_prices(parts: i32, summ: i32) -> Vec<i32>
    {
        logger::StructLogger::new_default();
        let mut rng = rand::thread_rng();
        let parts_val = summ / parts;
        let mut prices = vec![];
        for _ in 0..parts
        {
            prices.push(parts_val);
        }
        let mut current_percent = rng.gen_range(0.05..=0.15);
        let prices_len = prices.len();
        for (i, p) in prices.iter_mut().enumerate()
        {
            if i < prices_len
            {
                if i % 2 == 0
                {
                    current_percent = rng.gen_range(0.05..=0.15);
                    let new = ((*p as f32) * current_percent).ceil() as i32;
                    *p = *p + new;
                }   
                else 
                {
                    let new = ((*p as f32) * current_percent).ceil() as i32;
                    *p = *p - new;
                }
            }
        }
        let sum: i32 = prices.iter().sum();
        let minus = sum - summ;
        logger::debug!("sum: {}, source: {}, ,minus: {}, arr: {:?}", sum, summ, minus, &prices);
        *prices.last_mut().unwrap() -= minus;
        let sum: i32 = prices.iter().sum();
        logger::debug!("{:?}, {}", &prices, sum);
        prices
    }

    fn get_prices2(parts: i32, summ: i32) -> Vec<i32> 
    {
        use rand::Rng;
    
        let mut rng = rand::thread_rng();
        let parts_val = summ / parts; // Среднее значение
        let mut prices = vec![parts_val; parts as usize]; // Инициализируем массив
    
        // Генерация отклонений
        for i in 0..parts as usize 
        {
            let percent = rng.gen_range(0.05..=0.15);
            let adjustment = ((prices[i] as f32) * percent).ceil() as i32;
    
            if i % 2 == 0 {
                prices[i] += adjustment; // Увеличение
            } else {
                prices[i] -= adjustment; // Уменьшение
            }
        }
    
        // Проверяем, чтобы сумма не равнялась parts_val
        for price in &mut prices {
            if *price == parts_val {
                *price += rng.gen_range(1..=2); // Делаем небольшую корректировку
            }
        }
    
        // Корректируем последний элемент
        let current_sum: i32 = prices.iter().sum();
        let difference = current_sum - summ;
       
        if difference != 0 {
            let last_index = prices.len() - 1;
            prices[last_index] -= difference; // Исправляем сумму
        }
    
        // Проверяем, что все элементы соответствуют ограничениям
        for price in &prices {
            let percent_diff = ((price - parts_val).abs() as f32) / (parts_val as f32);
            if percent_diff < 0.05 || percent_diff > 0.15 {
                panic!("Элемент {} выходит за пределы ±15%", price);
            }
        }
    
        prices
    }

    ///`parts`  количество частей  
    /// `summ` общая сумма  
    /// `min_percent` минимальный процент отклонений
    /// `max_percent` максимальный процент отклонений
    /// `min_sum` минимальная сумма позиции, меньше которой отклонение цен делать нельзя
    fn split_orders_by_total_sum(parts: i32, summ: i32, min_percent: u8, max_percent:u8, min_sum: i32) -> Result<Vec<i32>, String> 
    {
        use rand::Rng;
        let min_percent_f = min_percent as f32 / 100.0;
        let max_percent_f = max_percent as f32 / 100.0;
        let parts_val = summ / parts;
        let mut prices = vec![parts_val; parts as usize];
        //минимальное скорректированное значение 
        let min_corrected_value = parts_val - (parts_val as f32 * min_percent_f).ceil() as i32;
        if min_corrected_value <= min_sum
        {
            return Err(format!("Среднее значение части {} не может быть меньше минимальной суммы {} - минимальный процент {} = {}", parts_val, min_sum, min_percent, min_corrected_value));
        }
        //максимальное скорректированное значение 
        let max_corrected_value = parts_val + (parts_val as f32 * min_percent_f) as i32;
       
        let calc_buf_val = |price :i32, buf: &mut i32| -> i32
        {
            let mut rng = rand::thread_rng();
            let mut percent = rng.gen_range(min_percent_f..=max_percent_f);
            let mut adjustment = ((price as f32) * percent).ceil() as i32;
            //logger::debug!("buffer: {}, prices[i]: {} ajustment: {} percent: {}", buf, &price, adjustment, percent);
            loop 
            {
                if buf.abs() < max_corrected_value 
                && (parts_val - adjustment) >= min_sum
                {
                    //logger::info!("looper return: {}", adjustment);
                    break adjustment;
                }
                else 
                {
                    percent = rng.gen_range(min_percent_f..=max_percent_f);
                    adjustment = ((price as f32) * percent).ceil() as i32;
                    //logger::info!("looper: buffer: {}, prices[i]: {} ajustment: {} percent: {}", buf, &price, adjustment, percent);
                }
            }
        };
        let mut buffer = 0i32;
        for i in 0..parts as usize 
        {
            let adjustment = calc_buf_val(prices[i], &mut buffer);

                if i as i32 == parts - 1
                {
                    if buffer.is_negative()
                    {
                        prices[i] += buffer.abs();
                        //logger::debug!("buffer: {}, prices[i]: {}", buffer, &prices[i]);
                    }
                    else 
                    {
                        prices[i] -= buffer;
                        //logger::debug!("buffer: {}, prices[i]: {}", buffer, &prices[i]);
                    }
                }
                else 
                {
                    if i % 2 == 0 
                    {
                        prices[i] += adjustment;
                        buffer += adjustment;
                        //logger::debug!("buffer: {}, prices[i]: {}", buffer, &prices[i]);
                    } 
                    else 
                    {
                        prices[i] -= adjustment;
                        buffer -= adjustment;
                        //logger::debug!("buffer: {}, prices[i]: {}", buffer, &prices[i]);
                    }
                }
            
            
        }
        // Корректируем последний элемент
        let current_sum: i32 = prices.iter().sum();
        let difference =  summ - current_sum;
        //logger::debug!(" buffer: {}, sum {} arr: {:?}",  buffer, current_sum, &prices);
        if difference != 0 
        {
            let p = prices.iter_mut().find(|f| (**f + difference) <= max_corrected_value);
            if let Some(p) = p
            {
                *p+=difference;
            }
        }
        let current_sum: i32 = prices.iter().sum();
        //logger::debug!(" buffer: {}, sum {} {} arr: {:?}",  buffer, current_sum, summ, &prices);
        Ok(prices)
    }

    
    ///`parts`  количество частей  
    /// `summ` общая сумма  
    /// `min_percent` минимальный процент отклонений
    /// `max_percent` максимальный процент отклонений
    /// `min_sum` минимальная сумма позиции, меньше которой отклонение цен делать нельзя
    fn split_orders_by_total_sum_2(parts: usize, total: f64) -> Vec<f64> 
    {
        use rand::Rng;
            let base_value = total / parts as f64; // Базовое значение для одной части
            let min = base_value * 0.85;           // Минимально допустимое значение
            let max = base_value * 1.15;           // Максимально допустимое значение
            let mut totals = Vec::new();
            let mut current_sum: f64;
            loop 
            {
                totals.clear(); // Очищаем массив перед каждой итерацией
                current_sum = 0.0;
        
                for _ in 0..parts - 1 
                {
                    let random_value = rand::thread_rng().gen_range(min..=max).round();
                    totals.push(random_value);
                    current_sum += random_value;
                }
        
                let last_value = (total - current_sum).round();
                totals.push(last_value);
                current_sum = totals.iter().sum();
        
                if current_sum == total 
                {
                    break;
                }
            }
            let current_sum: f64 = totals.iter().sum();
            //println!("{} {:?}", current_sum, totals);
            totals
           
    }
}