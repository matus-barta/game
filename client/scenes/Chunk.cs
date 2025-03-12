using System;
using System.Collections.Generic;
using System.Text.Json;
using System.Text.Json.Serialization;
using Godot;

namespace Data
{
	public class Chunk
	{
		public ulong id { get; set; }
		public string terrain_id { get; set; }
		public List<WorldObject> world_objects { get; set; }
		public List<Placable>? placables { get; set; }
		public static Chunk Deserialize(string json)
		{
			try
			{
				return JsonSerializer.Deserialize<Chunk>(json);
			}
			catch (Exception e)
			{
				GD.PrintErr(e.ToString());
			}
			return null;
		}

		public string GetModelUrl()
		{
			return "http://127.0.0.1:3000/assets/" + terrain_id;
		}
	}

	public interface IModelObject
	{
		public ulong id { get; set; }
		public string model_id { get; set; }
		public Vec3 position { get; set; }
		public Vec3 rotation { get; set; }

		public string GetModelUrl() { return "http://127.0.0.1:3000/assets/" + model_id; }
	}

	public class WorldObject : IModelObject
	{
		public ulong id { get; set; }
		public string model_id { get; set; }
		public Vec3 position { get; set; }
		public Vec3 rotation { get; set; }
		public List<Placable>? placables { get; set; }
	}

	public class Placable : IModelObject
	{
		public ulong id { get; set; }
		public string model_id { get; set; }
		public Vec3 position { get; set; }
		public Vec3 rotation { get; set; }
		public LightSource light_source { get; set; }
	}


	public class LightSource
	{
		public LightType light_type { get; set; }
		public Color color { get; set; }
		public float energy { get; set; }
		public float size { get; set; }
	}

	public enum LightType
	{
		SPOT,
		OMNI
	}
	public class Color
	{
		public float r { get; set; }
		public float g { get; set; }
		public float b { get; set; }

		public Godot.Color ToGodot()
		{
			return new Godot.Color(r, g, b);
		}
	}
	public class Vec3
	{
		public Vec3()
		{
			x = 0.0f;
			y = 0.0f;
			z = 0.0f;
		}
		public Vec3(float x, float y, float z)
		{
			this.x = x;
			this.y = y;
			this.z = z;
		}
		public float x { get; set; }
		public float y { get; set; }
		public float z { get; set; }

		public Vector3 ToGodot()
		{
			return new Vector3(x, y, z);
		}

		override public string ToString()
		{
			return "(" + x + ", " + y + ", " + z + ")";
		}

		public static Vec3 FromGodot(Vector3 vec3)
		{
			return new Vec3(vec3.X, vec3.Y, vec3.Z);
		}
	}

}
