@tool
class_name RemoteRealTransform extends Marker3D

@export var path: NodePath:
	set(value):
		path = value
		if is_node_ready():
			_update()

func _update():
	get_node(path).real_transform = self.global_transform

func _ready() -> void:
	set_notify_transform(true)
	_update()

func _notification(what: int) -> void:
	if what == NOTIFICATION_TRANSFORM_CHANGED:
		_update()
	
