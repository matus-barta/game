using System;
using System.Collections.Generic;
using System.Runtime.Serialization;
using System.Text.Json;
using Godot;

namespace Data
{
	public class Chunk
	{
		public UInt64 id { get; set; }
		public string terrain_id { get; set; }
		public List<WorldObject> world_objects { get; set; }
		public List<Placable>? placables { get; set; }
		public static Chunk Deserialize(string json)
		{
			return JsonSerializer.Deserialize<Chunk>(json);
		}

		public string GetModelUrl()
		{
			return "http://127.0.0.1:3000/assets/" + terrain_id;
		}
	}

	public class WorldObject
	{
		public UInt64 id { get; set; }
		public string model_id { get; set; }
		public Vec3 transform { get; set; }
		public Vec3 rotation { get; set; }
		public List<Placable>? placables { get; set; }

		public string GetModelUrl()
		{
			return "http://127.0.0.1:3000/assets/" + model_id;
		}
	}

	public class Placable
	{
		public UInt64 id { get; set; }
		public string model_id { get; set; }
		public Vec3 transform { get; set; }
		public Vec3 rotation { get; set; }

		public string GetModelUrl()
		{
			return "http://127.0.0.1:3000/assets/" + model_id;
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

		public static Vec3 FromGodot(Vector3 vec3)
		{
			return new Vec3(vec3.X, vec3.Y, vec3.Z);
		}
	}

}
