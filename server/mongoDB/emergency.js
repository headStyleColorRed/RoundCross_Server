// Emergency model
const mongoose = require("mongoose");
const User = require("./userModel")

const emergencySchema = new mongoose.Schema({
	owner: {
        type: Object,
        required: [true, "can't be blank"],
    },
    active: {
        type: Boolean,
        required: [true, "can't be blank"]
	},
    emergencyType: {
        type: String,
        required: [true, "can't be blank"]
	},
    answers: {
        type: Array,
        required: [true, "can't be blank"]
	},
    time: {
        type: String,
        required: [true, "can't be blank"]
	},
    localization: {
        type: Array,
        required: [true, "can't be blank"]
	},
    helped: {
        type: Boolean,
	},
    messages: {
        type: Object,
	},
	
});
const Emergency = mongoose.model("Emergency", emergencySchema);


module.exports = Emergency;