func get_first_score(user_obj) {
    // 展現函數接收物件，並進行陣列與字典聯合操作
    return user_obj.scores[0];
}

print("=== 建立複雜的 JSON 資料結構 ===");

user = {
    "name": "Alice",
    age: 25,
    scores:[100, 95, 80],
    "is_active": 1
};

print("使用者姓名:", user.name);
print("第一筆成績是:", get_first_score(user));

print("=== 開始修改資料 ===");
// 修改陣列裡面的元素
user.scores[2] = 99;
// 新增字典的鍵值
user.email = "alice@example.com";

print("修改後的成績陣列:", user.scores);
print("修改後的使用者物件:", user);