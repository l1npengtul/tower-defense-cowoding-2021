extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	self.connect("on_fire", get_parent(), "on_fire")
	pass # Replace with function body.

func on_fire():
	emit_signal("on_fire")
