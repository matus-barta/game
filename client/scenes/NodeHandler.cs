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
        GD.Print(modelObject.model_id);
        node.Name = modelObject.model_id;
        new HttpHandler(node).RequestModel(modelObject.GetModelUrl(), AddGltfMesh);
        node.GlobalPosition = modelObject.transform.ToGodot();

        return node;
    }

    public Node3D CreateModel(ulong id, string model_id, string url)
    {
        GD.Print(model_id);
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