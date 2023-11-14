## 详细了解HTTP请求
1. HTTP基本格式
```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```
名词解释：      
1. CRLF：代表回车和换行，这是打字机时代的术语！    
2. URI：统一资源标识符，与URL类似，HTTP规定使用URI。    

## 回应
1. 回应请求的基本格式
```Rust
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```