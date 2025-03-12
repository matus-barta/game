using Data;
using Godot;

public class NodeHandler
{
    private Node3D node;
    public NodeHandler(Node parentNode)
    {
        node = new Node3D();
        parentNode.AddChild(node);
    }

    public Node3D CreateModel(IModelObject modelObject)
    {

        GD.Print("Downloading model: " + modelObject.model_id + " with position: " + modelObject.position.ToString());

        if (modelObject.model_id != null)
        {
            node.Name = modelObject.model_id;
            new HttpHandler(node).RequestModel(modelObject.GetModelUrl(), AddGltfMesh);
        }
        else
        {
            node.Name = "light";
        }
        node.GlobalPosition = modelObject.position.ToGodot();
        node.GlobalRotation = modelObject.rotation.ToGodot();

        if (modelObject.GetType() == typeof(Placable))
        {
            var model = (Placable)modelObject;
            if (model.light_source != null)
            {
                GD.Print("Handling light");
                Light3D lightNode;
                switch (model.light_source.light_type)
                {
                    case LightType.SPOT:
                        lightNode = new SpotLight3D();
                        break;
                    case LightType.OMNI:
                        lightNode = new OmniLight3D();
                        break;
                    default:
                        GD.Print("Light without type!");
                        return node;
                }
                lightNode.LightColor = model.light_source.color.ToGodot();
                lightNode.LightEnergy = model.light_source.energy;
                lightNode.Name = "light_" + node.Name;
                node.AddChild(lightNode);
            }
        }
        return node;
    }

    public Node3D CreateModel(ulong id, string model_id, string url)
    {
        GD.Print("Downloading model: " + model_id);
        node.Name = model_id;
        new HttpHandler(node).RequestModel(url, AddGltfMesh);

        return node;
    }

    private void AddGltfMesh(byte[] gltfMesh)
    {
        var gltfDocumentLoad = new GltfDocument();
        var gltfStateLoad = new GltfState();
        var error = gltfDocumentLoad.AppendFromBuffer(gltfMesh, "", gltfStateLoad, 0);

        if (error == Error.Ok)
        {
            GD.Print("Loading model: " + node.Name);
            var gltfSceneRootNode = gltfDocumentLoad.GenerateScene(gltfStateLoad);
            gltfSceneRootNode.Name = "gltfModel_" + node.Name;
            node.AddChild(gltfSceneRootNode);
        }
        else
        {
            GD.PrintErr($"Couldn't load glTF scene (error code: {error}).");
        }
    }
}