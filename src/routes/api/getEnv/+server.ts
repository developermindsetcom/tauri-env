import { error, json } from '@sveltejs/kit';
import * as privateEnv from '$env/static/private';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({request}) => {
    const { name } = await request.json();
    const variable = (privateEnv as any)[name];

	return json({variable})
};