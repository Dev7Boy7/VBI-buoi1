# VBI-day1:

Bài tập 1 : Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
            let sub_arr = [6,8,10];


Bài tập 2 : Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho. 
Nâng cao hơn : Tìm kiếm không phân biêt chữ hoa thường, theo dạng regex.
https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.txt


Sửa bài tập 2 : do nếu dùng trim() sẽ trả về kết quá " x " giống "x" => contents.matches(&input_str[..input_str.len()-2]).count()
