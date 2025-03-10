using Godot;
using System.Text;

public partial class MainScene : Node3D
{
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		base._Ready();

		new HttpHandler(this).CreateHttpRequest(OnHandleChunk, "http://127.0.0.1:3000/chunk/1");
	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
	}

	private void OnHandleChunk(long result, long responseCode, string[] headers, byte[] body)
	{
		var chunk = Data.Chunk.Deserialize(Encoding.UTF8.GetString(body));
		GD.Print(chunk.terrain_id);

		var terrain = new Node3D();
		AddChild(terrain);
		terrain.Name = chunk.terrain_id;
		new HttpHandler(terrain).RequestModel(chunk.GetModelUrl());

		foreach (var worldObject in chunk.world_objects)
		{
			var wo = new Node3D();
			AddChild(wo);
			wo.Name = worldObject.model_id;
			new HttpHandler(wo).RequestModel(worldObject.GetModelUrl());
			wo.GlobalPosition = worldObject.transform.ToGodot();

			GD.Print(worldObject.model_id);
			if (worldObject.placables != null)
			{
				foreach (var placable in worldObject.placables)
				{
					GD.Print(placable.model_id);

					var placableNode = new Node3D();
					AddChild(placableNode);
					placableNode.Name = placable.model_id;
					new HttpHandler(placableNode).RequestModel(placable.GetModelUrl());
					placableNode.GlobalPosition = placable.transform.ToGodot();
				}
			}
		}

		if (chunk.placables != null)
		{
			foreach (var placable in chunk.placables)
			{
				GD.Print(placable.model_id);

				var placableNode = new Node3D();
				AddChild(placableNode);
				placableNode.Name = placable.model_id;
				new HttpHandler(placableNode).RequestModel(placable.GetModelUrl());
				placableNode.GlobalPosition = placable.transform.ToGodot();
			}
		}
	}
}
