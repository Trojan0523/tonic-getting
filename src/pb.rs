/*
 * @Author: BuXiongYu
 * @Date: 2024-10-28 13:44:07
 * @LastEditors: BuXiongYu
 * @LastEditTime: 2024-10-31 11:55:30
 * @Description: 请填写简介
 */

pub mod getting {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/getting.v1.rs"));
    }
}
