use std::sync::Arc;
use lancedb::connect;
use lancedb::connection::CreateTableMode;
// 修正點：使用 ExecutableQuery 而不是 Executable
use lancedb::query::{ExecutableQuery, QueryBase}; 
use arrow_array::{FixedSizeListArray, Int32Array, RecordBatch, RecordBatchIterator, StringArray, Float32Array, ArrayRef};
use arrow_schema::{DataType, Field, Schema};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 連線到本地資料庫目錄
    let uri = "data/sample_lancedb";
    let db = connect(uri).execute().await?;

    // 2. 定義資料結構 (Schema)
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("text", DataType::Utf8, false),
        Field::new("vector", DataType::FixedSizeList(
            Arc::new(Field::new("item", DataType::Float32, true)),
            3 
        ), false),
    ]));

    // 3. 準備原始資料
    let ids = Int32Array::from(vec![1, 2, 3]);
    let texts = StringArray::from(vec!["這是關於 Rust 的資料", "這是關於 Python 的資料", "這是關於 AI 的資料"]);
    let vector_values = Float32Array::from(vec![
        0.1, 0.2, 0.3, 
        0.9, 0.8, 0.7, 
        0.5, 0.5, 0.5,
    ]);

    let vector_field = Arc::new(Field::new("item", DataType::Float32, true));
    let vector_array = FixedSizeListArray::try_new(
        vector_field,
        3,
        Arc::new(vector_values) as ArrayRef,
        None
    )?;

    // 4. 打包資料
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(ids) as ArrayRef,
            Arc::new(texts) as ArrayRef,
            Arc::new(vector_array) as ArrayRef,
        ],
    )?;

    // 5. 建立資料表
    let table_name = "my_table";
    let batches = RecordBatchIterator::new(vec![Ok(batch)], schema.clone());
    
    let table = db
        .create_table(table_name, Box::new(batches))
        .mode(CreateTableMode::Overwrite)
        .execute()
        .await?;

    println!("資料表 '{}' 已建立。", table_name);

    // 6. 執行向量搜尋
    let query_vector = vec![0.1, 0.2, 0.35];
    println!("正在搜尋最接近 {:?} 的資料...", query_vector);

    // 這裡會用到 QueryBase 提供的 .limit() 和 ExecutableQuery 提供的 .execute()
    let mut stream = table
        .query()
        .nearest_to(query_vector)?
        .limit(2)
        .execute()
        .await?;

    println!("搜尋結果：");
    while let Some(batch) = stream.try_next().await? {
        // 印出 RecordBatch 的內容
        println!("{:?}", batch);
    }

    Ok(())
}