const express = require("express")
const router = express.Router()

// Modules
const User = require("../mongoDB/userModel.js")
const Emergency = require("../mongoDB/emergency.js")
const ValidationManager = require("../tools/validation.js")
const RequestManager = require("./requestManager")


router.post("/user_emergencies", async (req, res) => {
	let body = req.body
    let emergencies = new Array()

    // Validation
    let validationResult = ValidationManager.validateDataFields( body, ["email"], "getting user emergencies" );
    if (validationResult.isError)
        return res.status(200).send({ code: validationResult.error, status: validationResult.message });

	try {
        emergencies = await Emergency.find({ "owner.email" : body.email })
	} catch (err) {
		return res.status(200).send({ code: "400", status: "Error when searching for emergencies" }) 
	}

	res.status(200).send({ code: "200", status: "Searched emergencies success", data: emergencies})
});



module.exports = router;