<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use MessagePack\MessagePack;
use App\Models\Experience;

class ExperienceController extends Controller
{
    /**
     * Display a listing of the resource.
     */
    public function index()
    {
        $experiences = Experience::with('items')->get();
        $response = [];
        foreach ($experiences as $category) {
            $items = [];
            foreach ($category->items as $item) {
                $items[] = [
                    'name' => $item->name,
                    'items' => [$item->experience], // Ensure array structure
                ];
            }

            $response[] = [
                'title' => $category->name,
                'items' => $items,
            ];
        }

        $binaryData = MessagePack::pack($response);
        return response($binaryData, 200);
    }

    /**
     * Store a newly created resource in storage.
     */
    public function store(Request $request)
    {
        //
    }

    /**
     * Display the specified resource.
     */
    public function show(string $id)
    {
        //
    }

    /**
     * Update the specified resource in storage.
     */
    public function update(Request $request, string $id)
    {
        //
    }

    /**
     * Remove the specified resource from storage.
     */
    public function destroy(string $id)
    {
        //
    }
}
