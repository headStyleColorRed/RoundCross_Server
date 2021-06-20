// User.model.js
const mongoose = require("mongoose");
const userSchema = new mongoose.Schema({
	email: {
        type: String,
        required: [true, "can't be blank"],
        unique: true,
    },
    username: {
        type: String,
        required: [true, "can't be blank"]
	},
	nickName: {
		type: String,
        unique: true,
	},
	country: {
		type: String,
	},
	bikingModality: {
		type: String,
	}
	
});
const User = mongoose.model("User", userSchema);


module.exports = User;