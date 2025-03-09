using Godot;
using System;
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
		new HttpHandler(this).CreateHttpRequest(OnHandleTerrain, chunk.GetModelUrl());

		foreach (var worldObject in chunk.world_objects)
		{
			new HttpHandler(this).CreateHttpRequest(OnHandleWorldObject, chunk.GetModelUrl());
			GD.Print(worldObject.model_id);
			if (worldObject.placables != null)
			{
				foreach (var placable in worldObject.placables)
				{
					GD.Print(placable.model_id);
					new HttpHandler(this).CreateHttpRequest(OnHandlePlacable, placable.GetModelUrl());
				}
			}
		}

		if (chunk.placables != null)
		{
			foreach (var placable in chunk.placables)
			{
				GD.Print(placable.model_id);
				new HttpHandler(this).CreateHttpRequest(OnHandlePlacable, placable.GetModelUrl());
			}
		}
	}

	private void OnHandleTerrain(long result, long responseCode, string[] headers, byte[] body)
	{
		GD.Print("handle terrain data - size: " + body.Length);
	}

	private void OnHandleWorldObject(long result, long responseCode, string[] headers, byte[] body)
	{
		GD.Print("handle world object data - size: " + body.Length);
	}
	private void OnHandlePlacable(long result, long responseCode, string[] headers, byte[] body)
	{
		GD.Print("handle placable data - size: " + body.Length);
	}
}
