pub struct RankView {
}

#[derive(Debug)]
pub struct RankViewData {
    pub id: i32,
    pub content: String,
}

pub trait A {
    type DataType;

    fn refresh(&self, data: Self::DataType);
}

impl A for RankView {
    type DataType = RankViewData;

    fn refresh(&self, data: Self::DataType) {
        println!("refresh data is {:?}", data);
    }
}