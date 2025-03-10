using System.Text;
using Godot;

public class HttpHandler
{
    private Node parentNode;
    private HttpRequest httpRequest;
    private HttpRequest.RequestCompletedEventHandler reqCompletedEventHandler;
    public HttpHandler(Node parentNode)
    {
        this.parentNode = parentNode;
        httpRequest = new HttpRequest();
        this.parentNode.AddChild(httpRequest);
    }
    public void CreateHttpRequest(HttpRequest.RequestCompletedEventHandler reqCompletedEventHandler, string url)
    {
        this.reqCompletedEventHandler = reqCompletedEventHandler;
        httpRequest.RequestCompleted += InternalReq;
        httpRequest.Request(url);
    }

    public void RequestModel(string url)
    {
        httpRequest.RequestCompleted += ModelReq;
        httpRequest.Request(url);
    }

    private void InternalReq(long result, long responseCode, string[] headers, byte[] body)
    {
        reqCompletedEventHandler(result, responseCode, headers, body);
        httpRequest.RequestCompleted -= InternalReq;
        httpRequest.QueueFree();
    }

    private void ModelReq(long result, long responseCode, string[] headers, byte[] body)
    {
        AddGltfMesh(body);
        httpRequest.RequestCompleted -= ModelReq;
        httpRequest.QueueFree();
    }

    private void AddGltfMesh(byte[] gltfMesh)
    {
        var gltfDocumentLoad = new GltfDocument();
        var gltfStateLoad = new GltfState();
        var error = gltfDocumentLoad.AppendFromBuffer(gltfMesh, "", gltfStateLoad, 0);

        if (error == Error.Ok)
        {
            var gltfSceneRootNode = gltfDocumentLoad.GenerateScene(gltfStateLoad);
            parentNode.AddChild(gltfSceneRootNode);
        }
        else
        {
            GD.PrintErr($"Couldn't load glTF scene (error code: {error}).");
        }
    }
}