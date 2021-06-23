const express = require("express")
const router = express.Router()

// Modules
const User = require("../mongoDB/userModel.js")
const Emergency = require("../mongoDB/emergency.js")
const ValidationManager = require("../tools/validation.js")
const RequestManager = require("./requestManager")

router.post("/new_emergency", async (req, res) => {
	let body = req.body

    // Validation
    let validationResult = ValidationManager.validateDataFields( body, ["email", "active", "emergencyType", "answers", "time", 
                                                                        "localization", "helped", "messages"], "creating new emergency");
    if (validationResult.isError)
        return res.status(200).send({ code: validationResult.error, status: validationResult.message });

    let owner = await User.findOne({ email: body.email})
	// Create new emergency
	const emergency = new Emergency({owner: owner, active: body.active, emergencyType: body.emergencyType, 
                                answers: body.answers, time: body.time, localization: body.localization, helped: body.helped, messages: body.messages});

	try {
		await emergency.save().catch((err) => { throw err })
	} catch (err) {
		return res.status(200).send({ code: "400", status: err.code == 11000 ? "Emergency already exists" : err}) 
	}

	res.status(200).send({ code: "200", status: "Created new emergency Succesfully"})
});


router.post("/delete_emergency", async (req, res) => {
	let body = req.body

    // Validation
    let validationResult = ValidationManager.validateDataFields( body, ["emergencyId"], "deleting emergency");
    if (validationResult.isError)
        return res.status(200).send({ code: validationResult.error, status: validationResult.message });

	try {
		await Emergency.deleteOne({ _id: body.emergencyId }).catch((err) => { throw err })
	} catch (err) {
		return res.status(200).send({ code: "400", status: "Couldn't find emergency"}) 
	}

	res.status(200).send({ code: "200", status: "Delted new emergency Succesfully"})
});

module.exports = router;