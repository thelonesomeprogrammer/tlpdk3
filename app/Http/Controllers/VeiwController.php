<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
function dex() {
        $filePath = public_path() . '/files/index.html';
        return response()->file($filePath);
    }
class VeiwController extends Controller
{
    public function index() {
        return dex();
    }

    public function file($filename)
    {
        $filePath = public_path('files/' . $filename);

        if (file_exists($filePath)) {
            $extension = pathinfo($filePath, PATHINFO_EXTENSION);
            $mimeTypes = [
                'js'  => 'application/javascript',
                'css' => 'text/css',
            ];

            $mimeType = $mimeTypes[$extension] ?? mime_content_type($filePath);

            $mimeType = $mimeType ?: 'application/octet-stream';

            return response()->file($filePath, ['Content-Type' => $mimeType,]);
        }
        return dex();
    }
}
