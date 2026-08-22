function detectFields(){

return {

username:
document.querySelector(
'input[type="email"],input[name*=user]"
),

password:
document.querySelector(
'input[type="password"]'
)

};

}


function fill(username,password){

let fields = detectFields();

if(fields.username)
fields.username.value=username;

if(fields.password)
fields.password.value=password;

}
