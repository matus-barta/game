using Data;
using Godot;
using System.Text;
using System.Text.Json;

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
		var terrain = new NodeHandler(this).CreateModel(chunk.id, chunk.terrain_id, chunk.GetModelUrl());

		foreach (var worldObject in chunk.world_objects)
		{
			var wo = new NodeHandler(terrain).CreateModel(worldObject);

			if (worldObject.placables != null)
			{
				foreach (var placable in worldObject.placables)
				{
					new NodeHandler(wo).CreateModel(placable);
				}
			}
		}

		if (chunk.placables != null)
		{
			foreach (var placable in chunk.placables)
			{
				new NodeHandler(terrain).CreateModel(placable);
			}
		}
	}
}
