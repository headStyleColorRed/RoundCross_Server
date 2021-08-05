// User.model.js
const mongoose = require("mongoose");
const userSchema = new mongoose.Schema({
	email: {
		type: String,
		required: [true, "can't be blank"],
		unique: true,
	},
	username: {
		type: String
	},
	country: {
		type: String,
	},
	bikingModality: {
		type: Number,
	}

});
const User = mongoose.model("User", userSchema);


module.exports = User;