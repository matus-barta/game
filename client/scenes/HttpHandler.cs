using System;
using System.Text;
using Godot;

public class HttpHandler
{
    private Node parentNode;
    private HttpRequest httpRequest;
    private HttpRequest.RequestCompletedEventHandler reqCompletedEventHandler;
    private Action<byte[]> callback;
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

    public void RequestModel(string url, Action<byte[]> callback)
    {
        this.callback = callback;
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
        callback(body);
        httpRequest.RequestCompleted -= ModelReq;
        httpRequest.QueueFree();
    }
}